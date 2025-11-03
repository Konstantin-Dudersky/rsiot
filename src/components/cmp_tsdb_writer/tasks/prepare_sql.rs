use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use sqlx::{Connection, Pool, Postgres, query};
use tokio::{sync::mpsc, task::JoinHandle};
use tracing::warn;

use crate::executor::Instant;

use super::{Error, InnerMessage, QueryStat};

pub struct PrepareSQL {
    pub input: mpsc::Receiver<InnerMessage>,
    pub output: mpsc::Sender<JoinHandle<Result<QueryStat, Error>>>,
    pub table_name: &'static str,
    pub database_setup: Arc<AtomicBool>,
    pub db_pool: Pool<Postgres>,
    pub save_by_row_count: usize,
    pub save_by_period: Duration,
}

impl PrepareSQL {
    pub async fn spawn(mut self) -> Result<(), Error> {
        let cache_size = (self.save_by_row_count as f32 * 1.1) as usize;
        let mut cache = Vec::with_capacity(cache_size);

        let mut last_send = Instant::now();

        while let Some(msg) = self.input.recv().await {
            match msg {
                InnerMessage::Rows(rows) => {
                    cache.extend(rows);

                    if cache.len() < self.save_by_row_count {
                        continue;
                    }
                }
                InnerMessage::SendByTimer => {
                    if last_send.elapsed() < self.save_by_period {
                        continue;
                    }
                }
            }

            if cache.is_empty() {
                continue;
            }

            if self.output.capacity() <= 1 {
                warn!("ch_tx_database_to_results full, cancel sending");
                cache.clear();
                continue;
            }

            let mut query_stat = QueryStat::new();

            let sql = prepare_sql_statement(self.table_name, &cache)?;

            query_stat.set_rows_count(cache.len());
            query_stat.set_last_execution(last_send.elapsed());
            query_stat.set_sql_string_len(sql.len());

            last_send = Instant::now();
            cache.clear();

            if !self.database_setup.load(Ordering::Relaxed) {
                warn!("Database not setup");
                continue;
            }

            let task = execute_sql(self.db_pool.clone(), sql, query_stat);
            let task = tokio::task::Builder::new()
                .name("cmp_timescaledb | execute_sql")
                .spawn(task)
                .map_err(Error::Spawn)?;

            let res = self.output.try_send(task);
            if let Err(err) = res {
                warn!("Failed to send task to database: {}", err);
            }
        }
        Err(Error::TaskSendToDatabase)
    }
}

fn prepare_sql_statement(table_name: &str, rows: &[String]) -> Result<String, Error> {
    let values = rows.join(", ");

    let sql = format!(
        r#"INSERT INTO {table_name}
    VALUES {values}
    ON CONFLICT (time, prj, hst, svc, cmp, key) DO UPDATE
        SET value = excluded.value;"#
    );
    Ok(sql)
}

async fn execute_sql(
    pool: Pool<Postgres>,
    sql: String,
    query_stat: QueryStat,
) -> Result<QueryStat, Error> {
    let start = Instant::now();

    let mut conn = pool.acquire().await?;

    // Выполняем SQL-запрос
    query(&sql)
        .execute(&mut *conn)
        .await
        .map_err(Error::DatabaseExecute)?;

    // Закрываем соединение, иначе будет утечка памяти
    conn.detach().close().await?;

    let mut query_stat = query_stat;
    query_stat.set_execution_time(start.elapsed());

    Ok(query_stat)
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::{super::super::RowBuilder, *};

    #[test]
    fn test1() -> anyhow::Result<()> {
        let row_builder = RowBuilder::prj("prj_test")
            .hst("hst_test")
            .svc("svc_test")
            .cmp("cmp_test");

        let row1 = row_builder.row_with_ts("key1", 1.23, &datetime!(2025-07-23 10:00:00 +3))?;
        let row2 = row_builder.row_with_ts("key1", 4.56, &datetime!(2025-07-23 10:00:01 +3))?;

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
