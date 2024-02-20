use std::sync::Arc;

use axum::extract;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для получения всех сообщений
pub async fn list<TMsg>(
    extract::State(shared_state): extract::State<Arc<SharedState<TMsg>>>,
) -> Result<String, Error>
where
    TMsg: Clone,
{
    let mut msgs_json: Vec<String> = vec![];
    {
        let lock = shared_state.cache.read().await;
        for msg in lock.values() {
            let json = (shared_state.config.fn_input)(msg).map_err(Error::FnOutput)?;
            msgs_json.push(json);
        }
    }
    let json = msgs_json.join(",");
    let json = format!("[{}]", json);
    Ok(json)
}
