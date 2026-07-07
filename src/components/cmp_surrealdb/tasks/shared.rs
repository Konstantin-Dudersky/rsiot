use tracing::{trace, warn};

use crate::{
    components::cmp_surrealdb::config::{FnOnFailure, FnOnSuccess},
    executor::MsgBusOutput,
    message::MsgDataBound,
};

use super::{DB, Error};

pub async fn execute_db_query<TMsg>(
    msgbus_output: &MsgBusOutput<TMsg>,
    queries: Vec<String>,
    fn_on_success: FnOnSuccess<TMsg>,
    fn_on_failure: FnOnFailure<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    for (index, query) in queries.iter().enumerate() {
        trace!("Execute db query: {}", query);

        let mut response = DB.query(query).await?;

        let errors = response.take_errors();
        let msgs = match errors.is_empty() {
            true => {
                let on_success = fn_on_success(index, &mut response);
                match on_success {
                    Ok(msgs) => msgs,
                    Err(err) => {
                        let err = format!("Error in fn_on_success: {}", err);
                        warn!("{}", err);
                        return Ok(());
                    }
                }
            }
            false => {
                warn!("Response errors: {:?}", errors);
                fn_on_failure()
            }
        };
        for msg in msgs {
            msgbus_output
                .send(msg.to_message())
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }
    }

    Ok(())
}
