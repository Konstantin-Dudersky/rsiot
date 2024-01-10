use url::Url;

use rsiot_messages_core::IMessage;

/// Настройки Websocket-клиента
#[derive(Clone, Debug)]
pub struct Config<TMessage>
where
    TMessage: IMessage,
{
    /// Адрес Websocket-сервера
    pub url: Url,

    /// Преобразование входящих сообщений в текст для отправки на сервер
    ///
    /// По-умолчанию можно задать:
    ///
    /// ```rust
    /// |_: &TMessage| None
    /// ```
    pub fn_input: fn(&TMessage) -> Option<String>,

    /// Преобразование полученного от сервера текста в исходящие сообщения
    ///
    /// Пустой коллбек:
    ///
    /// ```rust
    /// |_: &str| Ok(vec![])
    /// ```
    /// 
    /// Для преобразования из json:
    /// 
    /// ```rust
    /// |text: &str| {
    ///     let msg = TMessage::from_json(text)?;
    ///     Ok(vec![msg])
    /// }
    /// ```
    pub fn_output: fn(&str) -> anyhow::Result<Vec<TMessage>>,
}
