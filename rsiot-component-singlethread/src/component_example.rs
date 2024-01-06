//! Пример "пустого" компонента. Для тестирования

use rsiot_messages_core::IMessage;

use crate::{error::ComponentError, Cache, Component, ComponentInput, ComponentOutput};

async fn fn_process<TMessage>(
    _input: ComponentInput<TMessage>,
    _output: ComponentOutput<TMessage>,
    _config: Config,
    _cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage,
{
    Ok(())
}

async fn fn_process2<TMessage>() -> Result<(), ComponentError>
where
    TMessage: IMessage,
{
    Ok(())
}

#[derive(Debug)]
pub struct Config {}

pub fn new<TMessage>(config: Config) -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    Box::new(Component::new(config, fn_process2))
}
