use url::Url;

use rsiot_messages_core::IMessage;

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
