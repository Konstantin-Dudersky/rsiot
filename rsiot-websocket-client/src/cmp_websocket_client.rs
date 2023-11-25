use rsiot_component_core::Component;
pub use rsiot_components_config::websocket_client::Config;
use rsiot_messages_core::IMessage;

use crate::function::function;

pub fn new<TMessage>(config: Config<TMessage>) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, function);
    Box::new(cmp)
}
