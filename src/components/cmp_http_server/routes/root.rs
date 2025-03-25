use axum::{extract, response::Html};

use crate::{components_config::http_server::handler_info, message::MsgDataBound};

use super::super::shared_state::SharedState;

/// Маршрут для получения сообщений
pub async fn root<TMsg>(
    extract::State(shared_state): extract::State<SharedState<TMsg>>,
) -> Html<String>
where
    TMsg: MsgDataBound,
{
    let get_endpoints = shared_state.get_endpoints.lock().await;
    let put_endpoints = shared_state.put_endpoints.lock().await;
    let body = handler_info(&get_endpoints, &put_endpoints);
    Html(body)
}
