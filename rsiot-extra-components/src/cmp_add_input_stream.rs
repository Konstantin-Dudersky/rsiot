//! Компонент для добавления сообщений из побочного потока

use tokio::sync::broadcast;

use rsiot_component_core::{Component, Input, Output};
use rsiot_messages_core::IMessage;

use super::cmpbase_many_mpsc_to_mpsc;

async fn fn_process<TMessage>(
    input: Input<TMessage>,
    output: Output<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    cmpbase_many_mpsc_to_mpsc::new(vec![input, config.channel], output).await;
}

/// Настройки
#[derive(Debug)]
pub struct Config<TMessage> {
    pub channel: broadcast::Receiver<TMessage>,
}

/// Компонент для добавления сообщений из побочного потока
pub fn new<TMessage>(config: Config<TMessage>) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
