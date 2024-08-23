use tracing::info;

use crate::{components::cmp_surrealdb::InputConfig, executor::CmpInOut, message::MsgDataBound};

use super::super::DbClient;

pub struct RequestInput<TMsg> {
    pub in_out: CmpInOut<TMsg>,
    pub input_config: InputConfig<TMsg>,
    pub db_client: DbClient,
}

impl<TMsg> RequestInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.in_out.recv_input().await {
            let query = (self.input_config.fn_input)(&msg);
            let query = match query {
                Some(val) => val,
                None => continue,
            };
            info!("Execute db query: {}", query);
            let db_client = self.db_client.lock().await;
            let response = db_client.query(query).await?;
            info!("Response: {:?}", response);
        }
        Ok(())
    }
}
