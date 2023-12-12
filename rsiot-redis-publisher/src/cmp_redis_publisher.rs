//! Компонент для публикации сообщений в Redis

use rsiot_component_core::Component;
pub use rsiot_components_config::redis_publisher::Config;
use rsiot_messages_core::IMessage;

use crate::function::function;

pub fn new<TMessage>(config: Config) -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, function);
    Box::new(cmp)
}
