use std::sync::Arc;

use axum::extract;

use rsiot_messages_core::MsgDataBound;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для ввода сообщений
pub async fn replace<TMsg>(
    extract::State(shared_state): extract::State<Arc<SharedState<TMsg>>>,
    body: String,
) -> Result<(), Error>
where
    TMsg: MsgDataBound,
{
    let msg = (shared_state.config.fn_output)(&body).map_err(Error::FnInput)?;
    let msg = match msg {
        Some(val) => val,
        None => return Ok(()),
    };
    shared_state
        .cmp_interface
        .send_output(msg)
        .await
        .map_err(Error::CmpOutput)?;
    Ok(())
}
