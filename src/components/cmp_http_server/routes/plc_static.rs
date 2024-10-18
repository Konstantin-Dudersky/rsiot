use std::sync::Arc;

use axum::extract;

use crate::message::*;

use super::super::{error::Error, shared_state::SharedState};

/// Маршрут для получения всех сообщений
pub async fn plc_static<TMsg>(
    extract::State(shared_state): extract::State<Arc<SharedState<TMsg>>>,
) -> Result<String, Error>
where
    TMsg: MsgDataBound,
{
    let Some(cmp_plc_static) = shared_state.config.cmp_plc_static.clone() else {
        return Err(Error::NotConfigured("plc_static".to_string()));
    };

    let msg = shared_state
        .msg_bus
        .recv_cache_msg(cmp_plc_static.key)
        .await
        .ok_or(Error::UnknownMessageKey(cmp_plc_static.key.to_string()))?;

    let data =
        (cmp_plc_static.fn_input)(&msg).ok_or(Error::NotConfigured("plc_static".to_string()))?;

    Ok(data)
}
