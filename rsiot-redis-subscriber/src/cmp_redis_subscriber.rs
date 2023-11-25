use rsiot_component_core::Component;
pub use rsiot_components_config::redis_subscriber::Config;
use rsiot_messages_core::IMessage;

use crate::function::function;

pub fn create<TMessage>(config: Config) -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, function);
    Box::new(cmp)
}
