use rsiot_component_core::Component;
use rsiot_messages_core::{IMessage, IMessageChannel};

use crate::{config::Config, fn_process::fn_process};

pub fn new<TMessage, TMessageChannel>(
    config: Config<TMessageChannel>,
) -> Box<Component<TMessage, Config<TMessageChannel>>>
where
    TMessage: IMessage + 'static,
    TMessageChannel: IMessageChannel + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
