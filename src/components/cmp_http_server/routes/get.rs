use axum::extract;
use axum::http::{HeaderMap, header};
use axum::http::{HeaderValue, Uri};
use tracing::warn;

use crate::message::*;

use super::super::shared_state::SharedState;

/// Маршрут для получения данных
pub async fn get<TMsg>(
    uri: Uri,
    extract::State(shared_state): extract::State<SharedState<TMsg>>,
) -> (HeaderMap, Result<Vec<u8>, super::Error>)
where
    TMsg: MsgDataBound,
{
    let path = uri.path();

    let mut headers = HeaderMap::new();

    let header_content_type = "text/plain; charset=utf-8".parse::<HeaderValue>();
    let header_content_type = match header_content_type {
        Ok(v) => v,
        Err(e) => {
            warn!("Error parsing header: {e}");
            return (headers, Err(super::Error::InvalidHeaderValue(e)));
        }
    };
    headers.insert(header::CONTENT_TYPE, header_content_type);

    let data = {
        let get_endpoints = shared_state.get_endpoints.lock().await;
        get_endpoints.handler(path, super::Error::UnknownPath, super::Error::Serde)
    };

    (headers, data)
}
