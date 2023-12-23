//! Преобразование mpsc::Receiver в broadcast::Sender

use tokio::sync::{
    broadcast::{self},
    mpsc,
};
use tracing::debug;

use rsiot_messages_core::IMessage;

/// Компонент для перенаправления сообщений из `tokio::sync::mpsc` в `tokio::sync::broadcast`
pub async fn new<TMessage>(mut input: mpsc::Receiver<TMessage>, output: broadcast::Sender<TMessage>)
where
    TMessage: IMessage,
{
    debug!("cmpbase_mpsc_to_broadcast started");
    while let Some(msg) = input.recv().await {
        output.send(msg).unwrap();
    }
}
