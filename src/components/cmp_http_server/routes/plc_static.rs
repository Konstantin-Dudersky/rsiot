use axum::extract;

use crate::message::*;

use super::super::{error::Error, shared_state::TSharedState};

/// Маршрут для получения всех сообщений
pub async fn plc_static<TMsg, TService>(
    extract::State(shared_state): extract::State<TSharedState<TMsg, TService>>,
) -> Result<String, Error>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    let shared_state = shared_state.lock().await;
    let data = shared_state.cmp_plc_static.clone();
    Ok(data)
}
