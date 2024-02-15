use std::sync::Arc;

use axum::extract;

use rsiot_messages_core::IMessage;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для получения всех сообщений
pub async fn list<TMessage>(
    extract::State(shared_state): extract::State<Arc<SharedState<TMessage>>>,
) -> Result<String, Error>
where
    TMessage: IMessage,
{
    let mut msgs_json: Vec<String> = vec![];
    {
        let lock = shared_state.cache.read().await;
        for msg in lock.values() {
            let json = (shared_state.config.fn_output)(msg).map_err(Error::FnOutput)?;
            msgs_json.push(json);
        }
    }
    let json = msgs_json.join(",");
    let json = format!("[{}]", json);
    Ok(json)
}
