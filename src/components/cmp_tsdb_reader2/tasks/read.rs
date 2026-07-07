use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use sqlx::{AssertSqlSafe, Pool, Postgres, query_as};
use tokio::time::sleep;
use tracing::info;

use crate::{
    executor::MsgBusOutput,
    message::{MsgDataBound, ValueTime},
};

use super::Error;

pub struct Read<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_output: MsgBusOutput<TMsg>,
    pub database_pool: Pool<Postgres>,
    pub delay_between_msgs: Duration,
    pub input_lagged: Arc<AtomicBool>,
}

impl<TMsg> Read<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn fetch(
        &mut self,
        sql: &str,
        fn_output: fn(ValueTime) -> TMsg,
    ) -> Result<(), Error> {
        let rows = query_as::<_, ValueTime>(AssertSqlSafe(sql))
            .fetch_all(&self.database_pool)
            .await
            .map_err(|e| Error::SqlxFetchAll(e.to_string()))?;

        for row in rows {
            let msg = (fn_output)(row);
            let msg = msg.to_message();

            if self.input_lagged.load(Ordering::Relaxed) {
                self.delay_between_msgs *= 2;
                sleep(Duration::from_millis(1_000)).await;
                self.input_lagged.store(false, Ordering::Release);
                info!("Change delay_between_msgs: {:?}", self.delay_between_msgs);
            }

            self.msgbus_output
                .send(msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;

            sleep(self.delay_between_msgs).await;
        }

        Ok(())
    }
}
