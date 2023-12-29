use std::sync::Arc;

use axum::extract;

use rsiot_messages_core::IMessage;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для получения всех сообщений
pub async fn list<TMessage>(
    extract::State(cache): extract::State<Arc<SharedState<TMessage>>>,
) -> Result<String, Error<TMessage>>
where
    TMessage: IMessage,
{
    let mut msgs_json: Vec<String> = vec![];
    {
        let lock = cache.cache.read().await;
        for msg in lock.values() {
            let msg_json = msg.to_json()?;
            msgs_json.push(msg_json);
        }
    }
    let json = msgs_json.join(",");
    let json = format!("[{}]", json);
    Ok(json)
}
