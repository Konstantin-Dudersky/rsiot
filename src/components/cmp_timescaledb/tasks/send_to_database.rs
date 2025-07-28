use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres};
use time::format_description::well_known::Iso8601;
use tokio::sync::mpsc;
use tokio_util::task::TaskTracker;
use tracing::{trace, warn};
use url::Url;

use super::{send_to_database_message::SendToDatabaseMessage, Error, Result, Row};

pub struct SendToDatabase {
    pub input: mpsc::Receiver<SendToDatabaseMessage>,
    pub connection_string: Url,
    pub max_connections: u32,
}

impl SendToDatabase {
    pub async fn spawn(mut self) -> Result<()> {
        let mut cache = vec![];

        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(self.connection_string.as_str())
            .await?;

        let task_tracker = TaskTracker::new();

        while let Some(msg) = self.input.recv().await {
            match msg {
                SendToDatabaseMessage::Rows(rows) => cache.extend(rows),
                SendToDatabaseMessage::SendByTimer => {
                    let sql = prepare_sql_statement(&cache)?;
                    let task = execute_sql(sql, pool.clone());
                    task_tracker.spawn(task);
                    cache.clear();
                }
            }
        }
        Err(Error::TaskSendToDatabase)
    }
}

fn prepare_sql_statement(rows: &[Row]) -> Result<String> {
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
        r#"INSERT INTO raw
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
    fn test1() {
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

        let test_sql = prepare_sql_statement(&rows).unwrap();
        let test_sql = test_sql
            .split('\n')
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join(" ");

        let correct_sql = "INSERT INTO raw VALUES ('2025-07-23T10:00:00.000000000+03:00', 'test_entity', 'test_attr', 1.23, 'Curr', NULL, ARRAY[]::AggType[]), ('2025-07-23T10:00:01.000000000+03:00', 'test_entity', 'test_attr', 4.56, 'Curr', NULL, ARRAY[]::AggType[]) ON CONFLICT (time, entity, attr, agg) DO UPDATE SET value = excluded.value, aggts = excluded.aggts, aggnext = excluded.aggnext;";

        assert_eq!(test_sql, correct_sql);
    }
}
