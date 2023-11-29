//! Пример "пустого" компонента. Для тестирования

use rsiot_messages_core::IMessage;

use crate::{Component, StreamInput, StreamOutput};

async fn process<TMessage>(
    _input: StreamInput<TMessage>,
    _output: StreamOutput<TMessage>,
    _config: Config,
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