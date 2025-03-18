use axum::extract;
use axum::http::Uri;

use crate::message::*;

use super::super::{error::Error, shared_state::TSharedState};

/// Маршрут для ввода данных от клиента
pub async fn put<TMsg, TService>(
    uri: Uri,
    extract::State(shared_state): extract::State<TSharedState<TMsg, TService>>,
    body: String,
) -> Result<(), Error>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    let path = uri.path();
    let shared_state = shared_state.lock().await;

    let msg = {
        let put_endpoints = shared_state.put_endpoints.lock().await;
        put_endpoints
            .get(path)
            .ok_or(Error::UnknownPath(path.to_string()))?
            .fn_output(&body)?
    };

    let Some(msg) = msg else { return Ok(()) };

    shared_state.msg_bus.send_output(msg).await?;

    Ok(())
}
