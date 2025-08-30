use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres};
use time::format_description::well_known::Iso8601;
use tokio::{sync::mpsc, task::JoinHandle};
use tracing::{info, trace, warn};
use url::Url;

use crate::executor::CheckCapacity;

use super::{Error, InnerMessage, Result, Row};

pub struct SendToDatabase {
    pub input: mpsc::Receiver<InnerMessage>,
    pub output: mpsc::Sender<JoinHandle<Result<()>>>,
    pub connection_string: Url,
    pub max_connections: u32,
    pub table_name: &'static str,
    pub delete_before_write: bool,
}

impl SendToDatabase {
    pub async fn spawn(mut self) -> Result<()> {
        let mut cache = vec![];

        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(self.connection_string.as_str())
            .await?;

        if self.delete_before_write {
            warn!("Deleting table {}", self.table_name);
            let sql = format!("DROP TABLE IF EXISTS {}", self.table_name);
            query(&sql).execute(&pool).await?;
        }

        if self.table_name != "raw" {
            info!("Creating table {}", self.table_name);
            let sql = format!(
                "CREATE TABLE IF NOT EXISTS {} (LIKE raw INCLUDING ALL)",
                self.table_name
            );
            query(&sql).execute(&pool).await?;
        }

        while let Some(msg) = self.input.recv().await {
            match msg {
                InnerMessage::Rows(rows) => cache.extend(rows),
                InnerMessage::SendByTimer => {
                    if cache.is_empty() {
                        continue;
                    }
                    let sql = prepare_sql_statement(self.table_name, &cache)?;
                    let task = execute_sql(sql, pool.clone());
                    cache.clear();
                    let task = tokio::task::Builder::new()
                        .name("cmp_timescaledb | execute_sql")
                        .spawn(task)
                        .map_err(Error::Spawn)?;
                    self.output
                        .check_capacity(0.2, "ch_tx_database_to_results")
                        .send_timeout(task, Duration::from_secs(5))
                        .await
                        .map_err(|_| Error::TokioMpsc)?;
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
            let aggts = match row.aggts {
                Some(v) => v.format(&Iso8601::DEFAULT)?,
                None => "NULL".to_string(),
            };
            let sql = format!(
                "('{time}', '{}', '{}', {}, '{:?}', {aggts}, ARRAY[]::AggType[])",
                row.entity, row.attr, row.value, row.agg,
            );
            Ok(sql)
        })
        .collect();
    let values = values?.join(", ");

    let sql = format!(
        r#"INSERT INTO {table_name}
    VALUES {values}
    ON CONFLICT (time, entity, attr, agg) DO UPDATE
        SET value = excluded.value,
             aggts = excluded.aggts,
             aggnext = excluded.aggnext;"#
    );
    Ok(sql)
}

async fn execute_sql(sql: String, pool: Pool<Postgres>) -> Result<()> {
    trace!("Execute SQL: {:?}", sql);
    let result = query(&sql).execute(&pool).await;
    if let Err(e) = result {
        warn!("Failed to execute SQL: {:?}", e);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::{super::super::AggType, *};

    #[test]
    fn test1() -> anyhow::Result<()> {
        let time1 = datetime!(2025-07-23 10:00:00 +3);
        let time2 = datetime!(2025-07-23 10:00:01 +3);
        let rows = vec![
            Row {
                time: time1,
                entity: "test_entity".to_string(),
                attr: "test_attr".to_string(),
                value: 1.23,
                agg: AggType::Curr,
                aggts: None,
                aggnext: vec![],
            },
            Row {
                time: time2,
                entity: "test_entity".to_string(),
                attr: "test_attr".to_string(),
                value: 4.56,
                agg: AggType::Curr,
                aggts: None,
                aggnext: vec![],
            },
        ];

        let test_sql = prepare_sql_statement("raw", &rows)?;
        let test_sql = test_sql
            .split('\n')
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join(" ");

        let correct_sql = "INSERT INTO raw VALUES ('2025-07-23T10:00:00.000000000+03:00', 'test_entity', 'test_attr', 1.23, 'Curr', NULL, ARRAY[]::AggType[]), ('2025-07-23T10:00:01.000000000+03:00', 'test_entity', 'test_attr', 4.56, 'Curr', NULL, ARRAY[]::AggType[]) ON CONFLICT (time, entity, attr, agg) DO UPDATE SET value = excluded.value, aggts = excluded.aggts, aggnext = excluded.aggnext;";

        assert_eq!(test_sql, correct_sql);
        Ok(())
    }
}
