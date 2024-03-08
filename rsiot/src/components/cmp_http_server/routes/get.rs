use std::sync::Arc;

use axum::extract;

use crate::message::*;

use super::super::{error::Error, shared_state::SharedState};

/// Маршрут для получения сообщений
pub async fn get<TMsg>(
    extract::Path(key): extract::Path<String>,
    extract::State(shared_state): extract::State<Arc<SharedState<TMsg>>>,
) -> Result<String, Error>
where
    TMsg: MsgDataBound,
{
    let msg = shared_state
        .cmp_interface
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
