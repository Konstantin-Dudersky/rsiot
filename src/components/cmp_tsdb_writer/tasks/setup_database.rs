use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use sqlx::{Connection, PgConnection, query};
use tokio::time::sleep;
use tracing::{info, warn};

use super::Error;

pub struct SetupDatabase {
    pub connection_string: String,
    pub reconnect_interval: Duration,
    pub delete_before_write: bool,
    pub table_name: &'static str,
    pub database_setup: Arc<AtomicBool>,
}

impl SetupDatabase {
    pub async fn spawn(self) -> Result<(), Error> {
        loop {
            let res = try_connect(
                &self.connection_string,
                self.delete_before_write,
                self.table_name,
            )
            .await;

            match res {
                Ok(_) => break,
                Err(e) => {
                    warn!("Failed to connect to database: {}", e);
                    sleep(self.reconnect_interval).await;
                    continue;
                }
            }
        }

        self.database_setup.store(true, Ordering::Release);

        Ok(())
    }
}

async fn try_connect(
    connection_string: &str,
    delete_before_write: bool,
    table_name: &str,
) -> Result<(), Error> {
    // Подключаемся к базе данных.
    //
    // Не используется пул подключений, поскольку была утечка памяти
    let mut conn = PgConnection::connect(connection_string)
        .await
        .map_err(Error::DatabaseConnect)?;

    if delete_before_write {
        warn!("Deleting table {}", table_name);
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        query(&sql).execute(&mut conn).await?;
    }

    if table_name != "raw" {
        info!("Creating table {}", table_name);
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (LIKE raw INCLUDING ALL)",
            table_name
        );
        query(&sql).execute(&mut conn).await?;
    }

    conn.close().await.map_err(Error::DatabaseCloseConnection)?;

    Ok(())
}
