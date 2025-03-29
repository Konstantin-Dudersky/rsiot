use axum::http::Uri;
use axum::{body, extract};

use crate::message::*;

use super::super::shared_state::SharedState;

/// Маршрут для ввода данных от клиента
pub async fn put<TMsg>(
    uri: Uri,
    extract::State(shared_state): extract::State<SharedState<TMsg>>,
    body: body::Bytes,
) -> Result<(), super::Error>
where
    TMsg: MsgDataBound,
{
    let path = uri.path();
    let msg = {
        let put_endpoints = shared_state.put_endpoints.lock().await;
        put_endpoints.handler(path, &body, super::Error::UnknownPath, super::Error::Serde)?
    };
    let Some(msg) = msg else { return Ok(()) };
    shared_state.msg_bus.send_output(msg).await?;
    Ok(())
}
