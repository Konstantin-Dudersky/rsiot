use rsiot_messages_core::IMessage;

/// Конфигурация компонента http-server
#[derive(Clone, Debug)]
pub struct Config<TMsg>
where
    TMsg: IMessage,
{
    /// Порт, через который доступен сервер
    pub port: u16,

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
    pub fn_input: fn(&str) -> anyhow::Result<Option<TMsg>>,

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
    pub fn_output: fn(&TMsg) -> anyhow::Result<String>,
}

// TODO - переименовать fn_input и fn_output - смысл наоборот