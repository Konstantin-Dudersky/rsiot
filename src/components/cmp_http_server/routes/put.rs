use axum::extract;
use axum::http::Uri;

use crate::{components_config::http_server::handler_put, message::*};

use super::super::shared_state::SharedState;

/// Маршрут для ввода данных от клиента
pub async fn put<TMsg>(
    uri: Uri,
    extract::State(shared_state): extract::State<SharedState<TMsg>>,
    body: String,
) -> Result<(), super::Error>
where
    TMsg: MsgDataBound,
{
    let path = uri.path();
    let msg = {
        let put_endpoints = shared_state.put_endpoints.lock().await;
        handler_put(
            path,
            &body,
            &put_endpoints,
            super::Error::UnknownPath,
            super::Error::JsonParseError,
        )?
    };
    let Some(msg) = msg else { return Ok(()) };
    shared_state.msg_bus.send_output(msg).await?;
    Ok(())
}
