//! Компонент для добавления сообщений из побочного потока

use tokio::sync::mpsc;

use rsiot_component_core::{Component, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

use super::cmpbase_many_mpsc_to_mpsc;

async fn fn_process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    cmpbase_many_mpsc_to_mpsc::new(vec![input, Some(config.channel)], output).await;
}

/// Настройки
#[derive(Debug)]
pub struct Config<TMessage> {
    pub channel: mpsc::Receiver<TMessage>,
}

pub fn new<TMessage>(config: Config<TMessage>) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
