use axum::extract;

use crate::message::*;

use super::super::{error::Error, shared_state::TSharedState};

/// Маршрут для получения всех сообщений
pub async fn plc_input<TMsg>(
    extract::State(shared_state): extract::State<TSharedState<TMsg>>,
) -> Result<String, Error>
where
    TMsg: MsgDataBound,
{
    let shared_state = shared_state.lock().await;

    // let Some(cmp_plc_input) = shared_state.config.cmp_plc_input.clone() else {
    //     return Err(Error::NotConfigured("plc_input".to_string()));
    // };

    // let msg = shared_state
    //     .msg_bus
    //     .recv_cache_msg(cmp_plc_input.key)
    //     .await
    //     .ok_or(Error::UnknownMessageKey(cmp_plc_input.key.to_string()))?;

    // let data =
    //     (cmp_plc_input.fn_input)(&msg).ok_or(Error::NotConfigured("plc_input".to_string()))?;

    let data = shared_state.cmp_plc_input.clone();

    Ok(data)
}
