//! Конфигурация websocket-клиента

use crate::message::Message;

/// Преобразование полученного от сервера текста в исходящие сообщения
pub type FnOutput<TMsg> = fn(&str) -> anyhow::Result<Option<Vec<Message<TMsg>>>>;

/// Настройки Websocket-клиента
#[derive(Clone, Debug)]
pub struct Config<TMsg> {
    /// Адрес Websocket-сервера
    ///
    /// "ws://localhost:9001"
    pub url: String,

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
    pub fn_output: FnOutput<TMsg>,

    /// Функция создает исходящее сообщение с информацией о соединении
    ///
    /// В функцию передается состояние соединения;
    /// - true - соединение установлено
    /// - false - соединение разорвано
    ///
    /// Примеры см. в тестах
    pub fn_connection_state: fn(bool) -> Option<Message<TMsg>>,
}

impl<TMsg> Default for Config<TMsg> {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            fn_input: |_| Ok(None),
            fn_output: |_| Ok(None),
            fn_connection_state: |_| None,
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use crate::message::example_message::*;

    #[test]
    fn fn_connection_state() {
        // Заглушка
        let fn_connection_state_1 = |_| None;

        let mut config_1: Config<Custom> = Default::default();
        config_1.fn_connection_state = fn_connection_state_1;
    }
}
