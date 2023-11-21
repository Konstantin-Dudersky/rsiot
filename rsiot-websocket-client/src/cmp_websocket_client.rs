use url::Url;

use rsiot_component_core::Component;
use rsiot_messages_core::IMessage;

use crate::function::function;

#[derive(Clone, Debug)]
pub struct Config<TMessage>
where
    TMessage: IMessage,
{
    /// Адрес сервера
    pub url: Url,
    /// stream_input -> передача на сервер
    pub fn_send: fn(TMessage) -> Option<String>,
    /// Данные от сервера -> stream_output
    pub fn_recv: fn(String) -> Vec<TMessage>,
}

pub fn new<TMessage>(
    config: Config<TMessage>,
) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, function);
    Box::new(cmp)
}
