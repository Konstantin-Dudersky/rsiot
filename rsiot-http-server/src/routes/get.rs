use std::sync::Arc;

use axum::extract;

use rsiot_messages_core::IMessage;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для получения сообщений
pub async fn get<TMessage>(
    extract::Path(key): extract::Path<String>,
    extract::State(shared_state): extract::State<Arc<SharedState<TMessage>>>,
) -> Result<String, Error>
where
    TMessage: IMessage,
{
    let msg;
    {
        let lock = shared_state.cache.read().await;
        msg = lock.get(&key).map(|m| m.to_owned());
    }
    let msg = msg.ok_or(Error::UnknownMessageKey(key))?;
    let json = (shared_state.config.fn_output)(&msg).map_err(Error::FnOutput)?;
    Ok(json)
}
