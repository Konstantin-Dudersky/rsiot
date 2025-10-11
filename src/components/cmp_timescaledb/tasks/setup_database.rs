use sqlx::{Pool, Postgres, postgres::PgPoolOptions, query};
use tracing::{info, warn};
use url::Url;

use super::{DatabasePool, Error};

pub struct SetupDatabase {
    pub pool: DatabasePool,
    pub max_connections: u32,
    pub connection_string: Url,
    pub delete_before_write: bool,
    pub table_name: &'static str,
}

impl SetupDatabase {
    pub async fn spawn(self) -> Result<(), Error> {
        let pool = loop {
            let pool = try_connect(
                self.max_connections,
                &self.connection_string,
                self.delete_before_write,
                self.table_name,
            )
            .await;

            match pool {
                Ok(v) => break v,
                Err(e) => {
                    warn!("Failed to connect to database: {}", e);
                    continue;
                }
            }
        };

        let mut lock = self.pool.lock().await;
        *lock = Some(pool);

        Ok(())
    }
}

async fn try_connect(
    max_connections: u32,
    connection_string: &Url,
    delete_before_write: bool,
    table_name: &str,
) -> Result<Pool<Postgres>, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(connection_string.as_str())
        .await?;

    if delete_before_write {
        warn!("Deleting table {}", table_name);
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        query(&sql).execute(&pool).await?;
    }

    if table_name != "raw" {
        info!("Creating table {}", table_name);
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (LIKE raw INCLUDING ALL)",
            table_name
        );
        query(&sql).execute(&pool).await?;
    }

    Ok(pool)
}
