use std::sync::Arc;

use axum::extract;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для получения сообщений
pub async fn get<TMsg>(
    extract::Path(key): extract::Path<String>,
    extract::State(shared_state): extract::State<Arc<SharedState<TMsg>>>,
) -> Result<String, Error>
where
    TMsg: Clone,
{
    let msg;
    {
        let lock = shared_state.cache.read().await;
        msg = lock.get(&key).map(|m| m.to_owned());
    }
    let msg = msg.ok_or(Error::UnknownMessageKey(key))?;
    let json = (shared_state.config.fn_input)(&msg).map_err(Error::FnOutput)?;
    Ok(json)
}
