use crate::{
    components::cmp_surrealdb::RequestStartConfig, executor::CmpInOut, message::MsgDataBound,
};

use super::{super::DbClient, shared::execute_db_query};

pub struct RequestStart<TMsg>
where
    TMsg: MsgDataBound,
{
    pub in_out: CmpInOut<TMsg>,
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
            self.in_out.clone(),
            &query,
            self.db_client.clone(),
            self.start_config.fn_on_success,
            self.start_config.fn_on_failure,
        )
        .await?;

        Ok(())
    }
}
