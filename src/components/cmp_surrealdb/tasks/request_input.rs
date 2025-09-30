use crate::{
    components::cmp_surrealdb::RequestInputConfig,
    executor::{MsgBusInput, MsgBusOutput},
    message::MsgDataBound,
};

use super::{super::DbClient, shared::execute_db_query};

pub struct RequestInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_input: MsgBusInput<TMsg>,
    pub msgbus_output: MsgBusOutput<TMsg>,
    pub input_config: RequestInputConfig<TMsg>,
    pub db_client: DbClient,
}

impl<TMsg> RequestInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.msgbus_input.recv().await {
            let query = (self.input_config.fn_input)(&msg);
            let query = match query {
                Some(val) => val,
                None => continue,
            };
            execute_db_query(
                &self.msgbus_output,
                &query,
                self.db_client.clone(),
                self.input_config.fn_on_success,
                self.input_config.fn_on_failure,
            )
            .await?;
        }
        Ok(())
    }
}
