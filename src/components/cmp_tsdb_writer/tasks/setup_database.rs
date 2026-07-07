use std::{str::FromStr, sync::Arc, time::Duration};

use sqlx::{
    AssertSqlSafe, ConnectOptions, Connection, Pool, Postgres,
    postgres::{PgConnectOptions, PgPoolOptions},
    query,
};
use tokio::{
    sync::Mutex,
    time::{sleep, timeout},
};
use tracing::{info, warn};

use super::{ConfigTableForSetup, Error};

pub struct SetupDatabase {
    pub connection_string: String,
    pub reconnect_interval: Duration,
    pub tables: Vec<ConfigTableForSetup>,
    pub db_pool_mutex: Arc<Mutex<Option<Pool<Postgres>>>>,
    pub max_connections: u32,
}

impl SetupDatabase {
    pub async fn spawn(self) -> Result<(), Error> {
        let pool = loop {
            let task = try_connect(&self.connection_string, &self.tables, self.max_connections);
            let res = timeout(Duration::from_millis(1_000), task).await;

            match res {
                Ok(Ok(pool)) => break pool,

                Ok(Err(e)) => {
                    warn!("Failed to setup database: {}", e);
                    sleep(self.reconnect_interval).await;
                    continue;
                }
                Err(e) => {
                    warn!("Failed to setup database: {}", e);
                    sleep(self.reconnect_interval).await;
                    continue;
                }
            }
        };

        {
            let mut lock = self.db_pool_mutex.lock().await;
            *lock = Some(pool);
        }

        info!("Database setup completed successfully");

        Ok(())
    }
}

async fn try_connect(
    connection_string: &str,
    tables: &[ConfigTableForSetup],
    max_connections: u32,
) -> Result<Pool<Postgres>, Error> {
    let conn_options = PgConnectOptions::from_str(connection_string)?
        .disable_statement_logging()
        // TODO - возможно вынести в конфигурацию
        .options([("statement_timeout", "1500")]) // в ms
        // TODO - возможно вынести в конфигурацию
        .application_name("cmp_tsdb_writer");
    let pool_options = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_slow_threshold(Duration::from_millis(1_000));
    info!("Trying to connect to database");
    let pool = pool_options.connect_with(conn_options).await?;
    info!("Connected to database successfully");

    let mut conn = pool.acquire().await?;

    for table in tables {
        // Удаляем таблицу, если задано в конфигурации
        if table.delete_before_write {
            warn!("Deleting table {}", table.table_name);
            let sql = format!("DROP TABLE IF EXISTS {}", table.table_name);
            query(AssertSqlSafe(sql))
                .execute(&mut *conn)
                .await
                .map_err(Error::DatabaseExecute)?;
        }

        info!("Creating table {}", table.table_name);
        let sql = sql_create_table(table);
        query(AssertSqlSafe(sql))
            .execute(&mut *conn)
            .await
            .map_err(Error::DatabaseExecute)?;

        info!("Remove columnstore property");
        let sql = sql_remove_columnstore_policy(table);
        query(AssertSqlSafe(sql))
            .execute(&mut *conn)
            .await
            .map_err(Error::DatabaseExecute)?;

        info!("Add columnstore property");
        let sql = sql_add_columnstore_policy(table);
        query(AssertSqlSafe(sql))
            .execute(&mut *conn)
            .await
            .map_err(Error::DatabaseExecute)?;

        info!("Remove retention policy");
        let sql = sql_remove_retention_policy(table);
        query(AssertSqlSafe(sql))
            .execute(&mut *conn)
            .await
            .map_err(Error::DatabaseExecute)?;

        info!("Add retention policy");
        let sql = sql_add_retention_policy(table);
        if let Some(sql) = sql {
            query(AssertSqlSafe(sql))
                .execute(&mut *conn)
                .await
                .map_err(Error::DatabaseExecute)?;
        }
    }

    info!("Trying to close connection to database");

    // Закрываем соединение с базой данных
    conn.detach().close().await?;

    Ok(pool)
}

fn sql_create_table(table: &ConfigTableForSetup) -> String {
    let values = table
        .values
        .iter()
        .map(|f| {
            format!(
                "    {} {} NULL,\n",
                f.field_name,
                f.data_type.into_pg_type()
            )
        })
        .collect::<Vec<String>>()
        .join("");

    let chunk_interval = format!("PT{}S", table.chunk_interval.as_secs());

    format!(
        r#"
CREATE TABLE IF NOT EXISTS "{}" (
    time        TIMESTAMPTZ         NOT NULL,
    {}
    UNIQUE (time)
) WITH (
    tsdb.hypertable,
    tsdb.partition_column = 'time',
    tsdb.chunk_interval = '{}',
    tsdb.orderby = 'time ASC'
);
"#,
        table.table_name, values, chunk_interval
    )
}

fn sql_remove_columnstore_policy(table: &ConfigTableForSetup) -> String {
    format!(
        r#"
CALL remove_columnstore_policy('{}', if_exists => true);"#,
        table.table_name
    )
}

fn sql_add_columnstore_policy(table: &ConfigTableForSetup) -> String {
    let interval = format!("PT{}S", table.compress_interval.as_secs());
    format!(
        r#"
CALL add_columnstore_policy('{}', after => INTERVAL '{}');"#,
        table.table_name, interval
    )
}

fn sql_remove_retention_policy(table: &ConfigTableForSetup) -> String {
    format!(
        r#"
SELECT remove_retention_policy('{}', if_exists => true);"#,
        table.table_name
    )
}

fn sql_add_retention_policy(table: &ConfigTableForSetup) -> Option<String> {
    let interval = format!("PT{}S", table.retention_interval?.as_secs());
    let sql = format!(
        r#"
SELECT add_retention_policy('{}', drop_after => INTERVAL '{}');"#,
        table.table_name, interval
    );
    Some(sql)
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::{ConfigTableField, ConfigTableFieldType},
        *,
    };

    use pretty_assertions::assert_eq;

    fn remove_spaces(text: &str) -> String {
        text.split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
            .split("\n")
            .collect::<Vec<&str>>()
            .join("")
    }

    #[test]
    fn test1() {
        let table1 = ConfigTableForSetup {
            table_name: "table1".into(),
            delete_before_write: false,
            chunk_interval: Duration::from_secs(60),
            compress_interval: Duration::from_secs(3600),
            retention_interval: None,
            values: vec![
                ConfigTableField {
                    field_name: "value_float".into(),
                    data_type: ConfigTableFieldType::NumericDoublePrecision,
                },
                ConfigTableField {
                    field_name: "value_int".into(),
                    data_type: ConfigTableFieldType::NumericInteger,
                },
            ],
        };

        let sql_1 = sql_create_table(&table1);
        let sql_1 = remove_spaces(&sql_1);

        let sql_2 = r#"
CREATE TABLE IF NOT EXISTS "table1" (
    time        TIMESTAMPTZ         NOT NULL,
    value_float DOUBLE PRECISION    NULL,
    value_int   INTEGER             NULL,
    UNIQUE (time)
) WITH (
    tsdb.hypertable,
    tsdb.partition_column = 'time',
    tsdb.chunk_interval = 'PT60S',
    tsdb.orderby = 'time ASC'
);"#;
        let sql_2 = remove_spaces(sql_2);

        assert_eq!(sql_1, sql_2);
    }
}
