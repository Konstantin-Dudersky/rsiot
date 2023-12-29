//! Компонент для публикации сообщений в Redis

use rsiot_component_core::Component;
use rsiot_messages_core::{IMessage, IMessageChannel};

use crate::{config::Config, fn_process::fn_process};

pub fn new<TMessage, TMessageChannel>(
    config: Config<TMessage, TMessageChannel>,
) -> Box<Component<TMessage, Config<TMessage, TMessageChannel>>>
where
    TMessage: IMessage + 'static,
    TMessageChannel: IMessageChannel + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
