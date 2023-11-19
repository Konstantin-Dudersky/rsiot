use url::Url;

use rsiot_component_core::Component;
use rsiot_messages_core::IMessage;

use crate::function::function;

#[derive(Clone)]
pub struct Config<TMessage> {
    /// Адрес сервера
    pub url: Url,
    /// stream_input -> передача на сервер
    pub fn_to_server: fn(TMessage) -> Option<String>,
    /// Данные от сервера -> stream_output
    pub fn_from_server: fn(String) -> Vec<TMessage>,
}

pub fn create<TMessage>(
    config: Config<TMessage>,
) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, function);
    Box::new(cmp)
}
