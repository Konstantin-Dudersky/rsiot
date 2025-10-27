use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use time::format_description::well_known::Iso8601;
use tokio::sync::mpsc;
use tracing::warn;

use crate::executor::CheckCapacity;

use super::{Error, InnerMessage, Result, Row};

pub struct PrepareSQL {
    pub input: mpsc::Receiver<InnerMessage>,
    pub output: mpsc::Sender<String>,
    pub max_cache_size: usize,
    pub table_name: &'static str,
    pub database_setup: Arc<AtomicBool>,
}

impl PrepareSQL {
    pub async fn spawn(mut self) -> Result<()> {
        let mut cache = Vec::with_capacity(self.max_cache_size);

        while let Some(msg) = self.input.recv().await {
            match msg {
                InnerMessage::Rows(rows) => {
                    cache.extend(rows);
                    if cache.len() > self.max_cache_size {
                        warn!(
                            "Кэш превысил максимальный размер: {}; очистка",
                            self.max_cache_size
                        );
                        cache.clear();
                    }
                }
                InnerMessage::SendByTimer => {
                    if cache.is_empty() {
                        continue;
                    }

                    if self.output.capacity() <= 1 {
                        warn!("ch_tx_database_to_results full, cancel sending");
                        cache.clear();
                        continue;
                    }

                    let sql = prepare_sql_statement(self.table_name, &cache)?;
                    cache.clear();

                    if !self.database_setup.load(Ordering::Relaxed) {
                        warn!("Database not setup");
                        continue;
                    }

                    let res = self
                        .output
                        .check_capacity(0.2, "ch_tx_database_to_results")
                        .send_timeout(sql, Duration::from_millis(100))
                        .await;
                    if let Err(err) = res {
                        warn!("Failed to send task to database: {}", err);
                    }
                }
            }
        }
        Err(Error::TaskSendToDatabase)
    }
}

fn prepare_sql_statement(table_name: &str, rows: &[Row]) -> Result<String> {
    let values: Result<Vec<String>> = rows
        .iter()
        .map(|row| {
            let time = row.time.format(&Iso8601::DEFAULT)?;
            let sql = format!(
                "('{time}', '{}', '{}', '{}', '{}', '{}', {})",
                row.prj, row.hst, row.svc, row.cmp, row.key, row.value
            );
            Ok(sql)
        })
        .collect();
    let values = values?.join(", ");

    let sql = format!(
        r#"INSERT INTO {table_name}
    VALUES {values}
    ON CONFLICT (time, prj, hst, svc, cmp, key) DO UPDATE
        SET value = excluded.value;"#
    );
    Ok(sql)
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::{super::super::RowBuilder, *};

    #[test]
    fn test1() -> anyhow::Result<()> {
        let row_builder = RowBuilder::new()
            .prj("prj_test")
            .hst("hst_test")
            .svc("svc_test")
            .cmp("cmp_test");

        let row1 = row_builder
            .clone()
            .key("key1")
            .value(1.23)
            .time(datetime!(2025-07-23 10:00:00 +3))
            .row()?;
        let row2 = row_builder
            .clone()
            .key("key1")
            .value(4.56)
            .time(datetime!(2025-07-23 10:00:01 +3))
            .row()?;

        let rows = vec![row1, row2];

        let test_sql = prepare_sql_statement("raw", &rows)?;
        let test_sql = test_sql
            .split('\n')
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join(" ");

        let correct_sql = "INSERT INTO raw VALUES ('2025-07-23T10:00:00.000000000+03:00', 'prj_test', 'hst_test', 'svc_test', 'cmp_test', 'key1', 1.23), ('2025-07-23T10:00:01.000000000+03:00', 'prj_test', 'hst_test', 'svc_test', 'cmp_test', 'key1', 4.56) ON CONFLICT (time, prj, hst, svc, cmp, key) DO UPDATE SET value = excluded.value;";

        assert_eq!(test_sql, correct_sql);
        Ok(())
    }
}
