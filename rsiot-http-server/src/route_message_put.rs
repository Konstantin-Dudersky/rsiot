use std::sync::Arc;

use axum::extract;
use tracing::info;

use rsiot_messages_core::IMessage;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для ввода сообщений
pub async fn route_message_put<TMessage>(
    extract::State(state): extract::State<Arc<SharedState<TMessage>>>,
    body: String,
) -> Result<(), Error<TMessage>>
where
    TMessage: IMessage,
{
    let msg = TMessage::from_json(&body)?;
    info!("Message: {:?}", msg);
    state.stream_output.send(msg).await?;
    Ok(())
}
