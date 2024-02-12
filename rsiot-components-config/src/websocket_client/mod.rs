use url::Url;

use rsiot_messages_core::IMessage;

/// Настройки Websocket-клиента
#[derive(Clone, Debug)]
pub struct Config<TMsg>
where
    TMsg: IMessage,
{
    /// Адрес Websocket-сервера
    pub url: Url,

    /// Преобразование входящих сообщений в текст для отправки на сервер
    ///
    /// # Примеры
    ///
    /// ## Пустой коллбек
    ///
    /// ```rust
    /// |_: &TMsg| Ok(None)
    /// ```
    ///
    /// ## Преобразование в json
    ///
    /// ```rust
    /// |msg: &TMsg| {
    ///     let text = msg.to_json()?;
    ///     Ok(Some(text))
    /// }
    /// ```
    pub fn_input: fn(&TMsg) -> anyhow::Result<Option<String>>,

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
    ///     let msg = Message::from_json(text)?;
    ///     Ok(vec![msg])
    /// }
    /// # ;
    /// ```
    pub fn_output: fn(&str) -> anyhow::Result<Vec<TMsg>>,
}
