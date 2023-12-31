use std::sync::Arc;

use axum::extract;

use rsiot_messages_core::IMessage;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для получения сообщений
pub async fn get<TMessage>(
    extract::Path(key): extract::Path<String>,
    extract::State(cache): extract::State<Arc<SharedState<TMessage>>>,
) -> Result<String, Error<TMessage>>
where
    TMessage: IMessage,
{
    let msg;
    {
        let lock = cache.cache.read().await;
        msg = lock.get(&key).map(|m| m.to_owned());
    }
    let msg = msg.ok_or(Error::UnknownMessageKey(key))?;
    let json = msg.to_json()?;
    Ok(json)
}
