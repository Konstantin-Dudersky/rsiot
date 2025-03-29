use axum::extract;
use axum::http::Uri;
use axum::http::{header, HeaderMap};

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
    headers.insert(
        header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );

    let data = {
        let get_endpoints = shared_state.get_endpoints.lock().await;
        get_endpoints.handler(path, super::Error::UnknownPath, super::Error::Serde)
    };

    (headers, data)
}
