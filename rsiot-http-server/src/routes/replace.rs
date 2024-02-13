use std::sync::Arc;

use axum::extract;

use rsiot_messages_core::IMessage;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для ввода сообщений
pub async fn replace<TMessage>(
    extract::State(shared_state): extract::State<Arc<SharedState<TMessage>>>,
    body: String,
) -> Result<(), Error<TMessage>>
where
    TMessage: IMessage,
{
    let msg = (shared_state.config.fn_input)(&body).map_err(Error::FnInput)?;
    let msg = match msg {
        Some(val) => val,
        None => return Ok(()),
    };
    shared_state.output.send(msg).await?;
    Ok(())
}
