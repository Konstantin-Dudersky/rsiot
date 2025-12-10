use std::{sync::Arc, time::Duration};

use sqlx::{Connection, Pool, Postgres, query};
use tokio::{
    sync::{Mutex, mpsc},
    task::JoinHandle,
};
use tracing::{trace, warn};

use crate::executor::Instant;

use super::{Error, InnerMessage, QueryStat};

pub struct PrepareSQL {
    pub input: mpsc::Receiver<InnerMessage>,
    pub output: mpsc::Sender<JoinHandle<Result<QueryStat, Error>>>,
    pub table_name: String,
    pub db_pool_mutex: Arc<Mutex<Option<Pool<Postgres>>>>,
    pub save_by_row_count: usize,
    pub save_by_period: Duration,
}

impl PrepareSQL {
    pub async fn spawn(mut self) -> Result<(), Error> {
        let cache_size = (self.save_by_row_count as f32 * 1.1) as usize;
        let mut cache = Vec::with_capacity(cache_size);

        let mut last_send = Instant::now();
        let mut db_pool = DbPool::Uninitialized;

        while let Some(msg) = self.input.recv().await {
            match msg {
                InnerMessage::Row(row) => {
                    cache.push(row);

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

            let sql = prepare_sql_statement(&self.table_name, &cache)?;
            trace!("SQL statement: {}", sql);

            query_stat.set_table_name(&self.table_name);
            query_stat.set_rows_count(cache.len());
            query_stat.set_last_execution(last_send.elapsed());
            query_stat.set_sql_string_len(sql.len());

            last_send = Instant::now();
            cache.clear();

            // Пробуем получить пул подключений к базе данных
            let db_pool_clone = match &db_pool {
                DbPool::Uninitialized => {
                    let lock = self.db_pool_mutex.lock().await;
                    match lock.clone() {
                        Some(v) => {
                            db_pool = DbPool::Initialized(v.clone());
                            v
                        }
                        None => {
                            warn!("Database not setup");
                            continue;
                        }
                    }
                }
                DbPool::Initialized(v) => v.clone(),
            };

            let task = execute_sql(db_pool_clone, sql, query_stat);
            let task_name = format!("cmp_timescaledb | execute_sql | {}", self.table_name);
            let task = tokio::task::Builder::new()
                .name(&task_name)
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

enum DbPool {
    Uninitialized,
    Initialized(Pool<Postgres>),
}

fn prepare_sql_statement(table_name: &str, rows: &[String]) -> Result<String, Error> {
    let values = rows.join(", ");

    let sql = format!(
        r#"INSERT INTO "{table_name}"
    VALUES {values};"#
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

    use super::{super::super::row_with_ts, *};

    use pretty_assertions::assert_eq;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let row1 = row_with_ts(&datetime!(2025-07-23 10:00:00 +3), &[1.23.to_string()])?;
        let row2 = row_with_ts(&datetime!(2025-07-23 10:00:01 +3), &[4.56.to_string()])?;

        let rows = vec![row1, row2];

        let test_sql = prepare_sql_statement("raw", &rows)?;
        let test_sql = test_sql
            .split('\n')
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join(" ");

        let correct_sql = "INSERT INTO \"raw\" VALUES ('2025-07-23T10:00:00.000000000+03:00', 1.23), ('2025-07-23T10:00:01.000000000+03:00', 4.56);";

        assert_eq!(test_sql, correct_sql);
        Ok(())
    }
}
