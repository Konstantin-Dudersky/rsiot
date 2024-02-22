//! Конфигурация Websocket-сервера
//!
//! Тестирование:
//!
//! ```bash
//! cargo test -p rsiot-components-config --doc websocket_server
//! ```

use rsiot_messages_core::*;

/// Конфигурация Websocket-сервера
#[derive(Clone, Debug)]
pub struct Config<TMsg> {
    /// Порт, через который доступен сервер
    pub port: u16,

    /// Функция преобразования входящих сообщений в текст для отправки клиенту
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// # use rsiot_components_config::websocket_server::Config;
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # Config::<Custom> {
    /// #    port: 8000,
    /// fn_input: |_| Ok(None),
    /// #    fn_output: |_| Ok(None)
    /// # };
    /// ```
    ///
    /// ## Сериализация в json
    ///
    /// ```
    /// # use rsiot_components_config::websocket_server::Config;
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # Config::<Custom> {
    /// #    port: 8000,
    /// fn_input: |msg: &Message<Custom>| {
    ///     let text = msg.serialize()?;
    ///     Ok(Some(text))
    /// }
    /// # ,
    /// #    fn_output: |_| Ok(None)
    /// # };
    /// ```
    ///
    pub fn_input: fn(&Message<TMsg>) -> anyhow::Result<Option<String>>,

    /// Функция преобразования текста, полученного от клиента, в исходящий поток сообщений
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// # use rsiot_components_config::websocket_server::Config;
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # Config::<Custom> {
    /// #    port: 8000,
    /// #    fn_input: |_| Ok(None),
    /// fn_output: |_| Ok(None)
    /// # };
    /// ```
    ///
    /// ## Десериализация из json:
    /// ```rust
    /// # use rsiot_components_config::websocket_server::Config;
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # Config::<Custom> {
    /// #    port: 8000,
    /// #    fn_input: |_| Ok(None),
    /// fn_output: |text: &str| {
    ///     let msg = Message::<Custom>::deserialize(text)?;
    ///     Ok::<Option<Vec<Message<Custom>>>, anyhow::Error>(Some(vec![msg]))
    /// }
    /// # };
    /// ```
    ///
    pub fn_output: fn(&str) -> anyhow::Result<Option<Vec<Message<TMsg>>>>,
}
