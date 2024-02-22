use url::Url;

use rsiot_messages_core::Message;

/// Настройки Websocket-клиента
#[derive(Clone, Debug)]
pub struct Config<TMsg> {
    /// Адрес Websocket-сервера
    pub url: Url,

    /// Преобразование входящих сообщений в текст для отправки на сервер
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// |_: &Message<TMsg>| Ok(None)
    /// ```
    ///
    /// ## Сериализация в json
    ///
    /// ```rust
    /// |msg: &Message<ExampleMessage>| {
    ///     let text = msg.serialize()?;
    ///     Ok(Some(text))
    /// }
    /// ```
    pub fn_input: fn(&Message<TMsg>) -> anyhow::Result<Option<String>>,

    /// Преобразование полученного от сервера текста в исходящие сообщения
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// |_: &str| Ok(vec![])
    /// ```
    ///
    /// ## Десериализация из json:
    ///
    /// ```rust
    /// # use rsiot_messages::ExampleMessage as Message;
    /// |text: &str| {
    ///     let msg = Message::deserialize(text)?;
    ///     Ok(Some(vec![msg]))
    /// }
    /// # ;
    /// ```
    pub fn_output: fn(&str) -> anyhow::Result<Option<Vec<Message<TMsg>>>>,
}
