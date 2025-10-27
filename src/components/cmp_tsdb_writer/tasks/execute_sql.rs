use sqlx::{Connection, PgConnection, query};
use tokio::sync::mpsc;
use tracing::warn;

use super::{Error, Result};

pub struct ExecuteSQL {
    pub input: mpsc::Receiver<String>,
    pub connection_string: String,
}

impl ExecuteSQL {
    pub async fn spawn(mut self) -> Result<()> {
        while let Some(sql) = self.input.recv().await {
            let res = execute_sql(sql, self.connection_string.clone()).await;
            // let task = tokio::task::Builder::new()
            //     .name("cmp_timescaledb | execute_sql")
            //     .spawn(task)
            //     .map_err(Error::Spawn)?;

            // if let Err(err) = res {
            //     info!("SQL execution result: {:?}", res);
            // }
            if let Err(err) = res {
                warn!("Error sending to database: {}", err);
            }
        }
        Err(Error::TaskSendToDatabase)
    }
}

async fn execute_sql(sql: String, connection_string: String) -> Result<()> {
    // Подключаемся к базе данных.
    //
    // Не используется пул подключений, поскольку была утечка памяти
    let mut conn = PgConnection::connect(&connection_string)
        .await
        .map_err(Error::DatabaseConnect)?;

    // Выполняем SQL-запрос
    query(&sql)
        .execute(&mut conn)
        .await
        .map_err(Error::DatabaseExecute)?;

    // Закрываем подключение к базе данных
    conn.close().await.map_err(Error::DatabaseCloseConnection)?;

    Ok(())
}
