//! Пример "пустого" компонента. Для тестирования

use rsiot_messages_core::IMessage;

use crate::{CacheType, Component, ComponentInput, ComponentOutput};

async fn process<TMessage>(
    _input: ComponentInput<TMessage>,
    _output: ComponentOutput<TMessage>,
    _config: Config,
    _cache: CacheType<TMessage>,
) where
    TMessage: IMessage,
{
}

#[derive(Clone, Debug)]
pub struct Config {}

pub fn new<TMessage>() -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    Box::new(Component::new(Config {}, process))
}
