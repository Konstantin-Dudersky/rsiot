use url::Url;

use rsiot_component_core::Component;
use rsiot_messages_core::IMessage;

use crate::function::function;

#[derive(Clone, Debug)]
pub struct Config {
    /// Адрес сервера Redis
    pub url: Url,
    /// Название канала Pub/Sub и хеша, где хранятся сообщения
    pub redis_channel: String,
}

pub fn create<TMessage>(config: Config) -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, function);
    Box::new(cmp)
}
