use rsiot_messages_core::message_v2::Message;

/// Конфигурация компонента http-server
#[derive(Clone, Debug)]
pub struct Config<TMsg>
where
    TMsg: Clone,
{
    /// Порт, через который доступен сервер
    pub port: u16,

    /// Функция преобразования сообщений в текст
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// # enum Message{}
    /// |_: &Message| Ok::<String, anyhow::Error>(String::from(""))
    /// # ;
    /// ```
    ///
    /// ## Преобразование в json
    ///
    /// ```rust
    /// # use rsiot_messages_core::{ExampleMessage as Message, IMessage};
    /// |msg: &Message| {
    ///     let text = msg.to_json()?;
    ///     Ok(text) as anyhow::Result<String>
    /// }
    /// # ;
    /// ```
    pub fn_input: fn(&Message<TMsg>) -> anyhow::Result<String>,

    /// Функция преобразования текста в сообщения
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// # enum Message{}
    /// |_: &str| Ok::<String, anyhow::Error>(String::from(""))
    /// # ;
    /// ```
    ///
    /// ## Десериализация из json
    ///
    /// ```rust
    /// # use rsiot_messages_core::{ExampleMessage as Message, IMessage};
    /// |text: &str| {
    ///     let msg = Message::from_json(text)?;
    ///     Ok::<Option<Message>, anyhow::Error>(Some(msg))
    /// }
    /// # ;
    /// ```
    pub fn_output: fn(&str) -> anyhow::Result<Option<Message<TMsg>>>,
}
