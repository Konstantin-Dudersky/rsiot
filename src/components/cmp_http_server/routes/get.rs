use axum::extract;
use axum::http::Uri;

use crate::{components_config::http_server::handler_get, message::*};

use super::super::shared_state::SharedState;

/// Маршрут для получения данных
pub async fn get<TMsg>(
    uri: Uri,
    extract::State(shared_state): extract::State<SharedState<TMsg>>,
) -> Result<String, super::Error>
where
    TMsg: MsgDataBound,
{
    let path = uri.path();
    let get_endpoints = shared_state.get_endpoints.lock().await;

    let data = handler_get(
        path,
        &get_endpoints,
        super::Error::UnknownPath,
        super::Error::JsonParseError,
    )?;

    Ok(data)
}
