use rsiot_component_core::Component;
use rsiot_messages_core::IMessage;

use crate::{config::Config, process::process};

pub fn new<TMessage>(config: Config) -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, process);
    Box::new(cmp)
}
