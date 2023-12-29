use rsiot_component_core::Component;
use rsiot_messages_core::IMessage;

use crate::{config::config::Config, fn_process::fn_process};

pub fn new<TMessage>(config: Config<TMessage>) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
