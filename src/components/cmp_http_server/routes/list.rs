use axum::extract;

use crate::message::*;

use super::super::{error::Error, shared_state::TSharedState};

/// Маршрут для получения всех сообщений
pub async fn list<TMsg, TService>(
    extract::State(shared_state): extract::State<TSharedState<TMsg, TService>>,
) -> Result<String, Error>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    let shared_state = shared_state.lock().await;

    let mut msgs_json: Vec<String> = vec![];
    let cache = shared_state.msg_bus.recv_cache_all().await;
    for msg in cache {
        let json = (shared_state.config.fn_input)(&msg).map_err(Error::FnOutput)?;
        let Some(json) = json else { continue };
        msgs_json.push(json);
    }
    drop(shared_state);

    let json = msgs_json.join(",");
    let json = format!("[{}]", json);
    Ok(json)
}
