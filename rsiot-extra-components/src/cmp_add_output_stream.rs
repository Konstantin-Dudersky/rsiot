//! Компонент для отправки сообщений в побочный потока

use tokio::sync::mpsc;

use rsiot_component_core::{CacheType, Component, ComponentError, ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

use super::cmpbase_mpsc_to_many_mpsc;

async fn fn_process<TMessage>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage>,
    _cache: CacheType<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage + 'static,
{
    cmpbase_mpsc_to_many_mpsc::new(input, vec![output, config.channel]).await;
    Ok(())
}

/// Настройки
#[derive(Debug)]
pub struct Config<TMessage> {
    pub channel: mpsc::Sender<TMessage>,
}

/// Компонент для отправки сообщений в побочный потока
pub fn new<TMessage>(config: Config<TMessage>) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
