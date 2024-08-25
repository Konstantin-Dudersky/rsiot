use tracing::{trace, warn};

use crate::{
    components::cmp_surrealdb::config::{FnOnFailure, FnOnSuccess},
    executor::CmpInOut,
    message::MsgDataBound,
};

use super::super::DbClient;

pub async fn execute_db_query<TMsg>(
    in_out: CmpInOut<TMsg>,
    query: &str,
    db_client: DbClient,
    fn_on_success: FnOnSuccess<TMsg>,
    fn_on_failure: FnOnFailure<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    trace!("Execute db query: {}", query);

    let db_client = db_client.lock().await;
    let mut response = db_client.query(query).await?;

    let errors = response.take_errors();
    let msgs = match errors.is_empty() {
        true => {
            let on_success = fn_on_success(response);
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
        in_out.send_output(msg).await.unwrap();
    }
    Ok(())
}
