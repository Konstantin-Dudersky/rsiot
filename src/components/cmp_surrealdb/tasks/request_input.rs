use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use tokio::time::sleep;

use crate::{
    components::cmp_surrealdb::RequestInputConfig,
    executor::{MsgBusInput, MsgBusOutput},
    message::MsgDataBound,
};

use super::shared::execute_db_query;

pub struct RequestInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_input: MsgBusInput<TMsg>,
    pub msgbus_output: MsgBusOutput<TMsg>,
    pub input_config: RequestInputConfig<TMsg>,
    pub connection_established: Arc<AtomicBool>,
}

impl<TMsg> RequestInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        // Ожидание установления соединения
        loop {
            if self.connection_established.load(Ordering::Relaxed) {
                break;
            }
            sleep(Duration::from_millis(100)).await;
        }

        while let Ok(msg) = self.msgbus_input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            let query = (self.input_config.fn_input)(&msg);
            if query.is_empty() {
                continue;
            }
            execute_db_query(
                &self.msgbus_output,
                query,
                self.input_config.fn_on_success,
                self.input_config.fn_on_failure,
            )
            .await?;
        }
        Ok(())
    }
}
