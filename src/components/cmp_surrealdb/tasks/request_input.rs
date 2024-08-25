use crate::{
    components::cmp_surrealdb::RequestInputConfig, executor::CmpInOut, message::MsgDataBound,
};

use super::{super::DbClient, shared::execute_db_query};

pub struct RequestInput<TMsg> {
    pub in_out: CmpInOut<TMsg>,
    pub input_config: RequestInputConfig<TMsg>,
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
            execute_db_query(
                self.in_out.clone(),
                &query,
                self.db_client.clone(),
                self.input_config.fn_on_success,
                self.input_config.fn_on_failure,
            )
            .await?;

            // trace!("Execute db query: {}", query);

            // let db_client = self.db_client.lock().await;
            // let mut response = db_client.query(query).await?;

            // let errors = response.take_errors();
            // let msgs = match errors.is_empty() {
            //     true => {
            //         let on_success = (self.input_config.fn_on_success)(response);
            //         match on_success {
            //             Ok(msgs) => msgs,
            //             Err(err) => {
            //                 let err = format!("Error in fn_on_success: {}", err);
            //                 warn!("{}", err);
            //                 continue;
            //             }
            //         }
            //     }
            //     false => {
            //         warn!("Response errors: {:?}", errors);
            //         (self.input_config.fn_on_failure)()
            //     }
            // };
            // for msg in msgs {
            //     self.in_out.send_output(msg).await.unwrap();
            // }
        }
        Ok(())
    }
}
