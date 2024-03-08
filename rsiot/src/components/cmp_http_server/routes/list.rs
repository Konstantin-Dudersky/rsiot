use std::sync::Arc;

use axum::extract;

use crate::message::*;

use super::super::{error::Error, shared_state::SharedState};

/// Маршрут для получения всех сообщений
pub async fn list<TMsg>(
    extract::State(shared_state): extract::State<Arc<SharedState<TMsg>>>,
) -> Result<String, Error>
where
    TMsg: MsgDataBound,
{
    let mut msgs_json: Vec<String> = vec![];
    let cache = shared_state.cmp_interface.recv_cache_all().await;
    for msg in cache {
        let json = (shared_state.config.fn_input)(&msg).map_err(Error::FnOutput)?;
        let Some(json) = json else { continue };
        msgs_json.push(json);
    }

    let json = msgs_json.join(",");
    let json = format!("[{}]", json);
    Ok(json)
}
