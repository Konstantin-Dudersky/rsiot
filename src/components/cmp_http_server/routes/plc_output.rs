use std::sync::Arc;

use axum::extract;

use crate::message::*;

use super::super::{error::Error, shared_state::SharedState};

/// Маршрут для получения всех сообщений
pub async fn plc_output<TMsg>(
    extract::State(shared_state): extract::State<Arc<SharedState<TMsg>>>,
) -> Result<String, Error>
where
    TMsg: MsgDataBound,
{
    let Some(cmp_plc_output) = shared_state.config.cmp_plc_output.clone() else {
        return Err(Error::NotConfigured("plc_output".to_string()));
    };

    let msg = shared_state
        .msg_bus
        .recv_cache_msg(cmp_plc_output.key)
        .await
        .ok_or(Error::UnknownMessageKey(cmp_plc_output.key.to_string()))?;

    let data =
        (cmp_plc_output.fn_input)(&msg).ok_or(Error::NotConfigured("plc_output".to_string()))?;

    Ok(data)
}
