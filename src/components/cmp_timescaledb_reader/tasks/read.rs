use std::{sync::Arc, time::Duration};

use futures::TryStreamExt;
use sqlx::{Pool, Postgres, query_as};
use time::{OffsetDateTime, format_description::well_known::Iso8601};
use tokio::{sync::Semaphore, time::sleep};

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound, ValueTime},
};

use super::Error;

pub struct Read<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msg_bus: CmpInOut<TMsg>,
    pub database_pool: Pool<Postgres>,
    pub concurrent_connections: Arc<Semaphore>,
    pub time_begin: OffsetDateTime,
    pub time_end: OffsetDateTime,
    pub entity: &'static str,
    pub attr: &'static str,
    pub fn_output: fn(ValueTime) -> TMsg,
    pub delay_between_msgs: Duration,
}

impl<TMsg> Read<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), Error> {
        let _permit = self.concurrent_connections.acquire().await?;

        let sql = prepare_sql_statement(self.time_begin, self.time_end, self.entity, self.attr)?;

        let mut rows = query_as::<_, ValueTime>(&sql).fetch(&self.database_pool);

        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|e| Error::TryNext(e.to_string()))?
        {
            let msg = (self.fn_output)(row);
            let msg = Message::new_custom(msg);

            self.msg_bus
                .send_output(msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;

            sleep(self.delay_between_msgs).await;
        }

        Ok(())
    }
}

fn prepare_sql_statement(
    time_begin: OffsetDateTime,
    time_end: OffsetDateTime,
    entity: &str,
    attribute: &str,
) -> Result<String, Error> {
    let time_start = time_begin.format(&Iso8601::DEFAULT)?;
    let time_stop = time_end.format(&Iso8601::DEFAULT)?;

    let sql = format!(
        r#"SELECT "time", value
FROM raw
WHERE (
    "time" BETWEEN '{time_start}' AND '{time_stop}'
	AND entity = '{entity}'
	AND attr = '{attribute}'
)
ORDER BY "time" ASC;"#
    );
    Ok(sql)
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let time_start = datetime!(2025-08-19 07:20:00.000+03);
        let time_stop = datetime!(2025-08-19 07:20:10.000+03);
        let entity = "accelerometer";
        let attr = "accel_x";

        let sql_test = prepare_sql_statement(time_start, time_stop, entity, attr)?;

        let sql = r#"SELECT "time", value
FROM raw
WHERE (
    "time" BETWEEN '2025-08-19T07:20:00.000000000+03:00' AND '2025-08-19T07:20:10.000000000+03:00'
	AND entity = 'accelerometer'
	AND attr = 'accel_x'
)
ORDER BY "time" ASC;"#;

        assert_eq!(sql_test, sql);
        Ok(())
    }
}
