use tracing::{trace, warn};

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
            trace!("Execute db query: {}", query);

            let db_client = self.db_client.lock().await;
            let mut response = db_client.query(query).await?;

            let errors = response.take_errors();
            let msgs = match errors.is_empty() {
                true => {
                    let r: Option<String> = response.take(0).unwrap();
                    (self.input_config.fn_on_success)(&r.unwrap())
                }
                false => {
                    warn!("Response errors: {:?}", errors);
                    (self.input_config.fn_on_failure)()
                }
            };
            for msg in msgs {
                self.in_out.send_output(msg).await.unwrap();
            }
        }
        Ok(())
    }
}
