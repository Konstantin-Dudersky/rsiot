use crate::{
    components::cmp_surrealdb::RequestStartConfig, executor::MsgBusOutput, message::MsgDataBound,
};

use super::{super::DbClient, shared::execute_db_query};

pub struct RequestStart<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_output: MsgBusOutput<TMsg>,
    pub start_config: RequestStartConfig<TMsg>,
    pub db_client: DbClient,
}

impl<TMsg> RequestStart<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        let query = self.start_config.query;

        execute_db_query(
            &self.msgbus_output,
            &query,
            self.db_client.clone(),
            self.start_config.fn_on_success,
            self.start_config.fn_on_failure,
        )
        .await?;

        Ok(())
    }
}
