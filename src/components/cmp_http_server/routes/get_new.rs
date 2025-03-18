use axum::extract;
use axum::http::Uri;

use crate::message::*;

use super::super::{error::Error, shared_state::TSharedState};

/// Маршрут для получения данных
pub async fn get_new<TMsg, TService>(
    uri: Uri,
    extract::State(shared_state): extract::State<TSharedState<TMsg, TService>>,
) -> Result<String, Error>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    let path = uri.path();
    let shared_state = shared_state.lock().await;
    let get_endpoints = shared_state.get_endpoints.lock().await;

    let endpoint = get_endpoints
        .get(path)
        .ok_or(Error::UnknownPath(path.to_string()))?
        .get_json_data()
        .unwrap();

    Ok(endpoint)
}
