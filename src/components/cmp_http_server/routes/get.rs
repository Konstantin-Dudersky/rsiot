use axum::extract;

use crate::message::*;

use super::super::{error::Error, shared_state::TSharedState};

/// Маршрут для получения сообщений
pub async fn get<TMsg>(
    extract::Path(key): extract::Path<String>,
    extract::State(shared_state): extract::State<TSharedState<TMsg>>,
) -> Result<String, Error>
where
    TMsg: MsgDataBound,
{
    let shared_state = shared_state.lock().await;

    let msg = shared_state
        .msg_bus
        .recv_cache_msg(&key)
        .await
        .ok_or(Error::UnknownMessageKey(key))?;
    let json = (shared_state.config.fn_input)(&msg).map_err(Error::FnOutput)?;
    let json = match json {
        Some(json) => json,
        None => return Ok("".into()),
    };
    Ok(json)
}
