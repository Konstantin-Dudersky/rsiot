use axum::extract;

use crate::message::{MsgDataBound, ServiceBound};

use super::super::{error::Error, shared_state::TSharedState};

/// Маршрут для ввода сообщений
pub async fn replace<TMsg, TService>(
    extract::State(shared_state): extract::State<TSharedState<TMsg, TService>>,
    body: String,
) -> Result<(), Error>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    let shared_state = shared_state.lock().await;

    let msg = (shared_state.config.fn_output)(&body).map_err(Error::FnInput)?;
    let msg = match msg {
        Some(val) => val,
        None => return Ok(()),
    };

    shared_state
        .msg_bus
        .send_output(msg)
        .await
        .map_err(Error::CmpOutput)?;

    Ok(())
}
