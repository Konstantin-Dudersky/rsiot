//! Конфигурация HTTP-сервера
//!
//! Тестирование:
//!
//! ```bash
//! cargo test -p rsiot-components-config --doc http_server
//! ```

use rsiot_messages_core::*;

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
    /// # use rsiot_components_config::http_server as cmp_http_server;
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # // insert from tests::stub
    /// # cmp_http_server::Config::<ExampleMessage> {
    /// #     port: 8000,
    /// fn_input: |_| Ok(None),
    /// #     fn_output: |_| Ok(None),
    /// # };
    /// ```
    ///
    /// ## Сериализация в json
    ///
    /// ```rust
    /// # use rsiot_components_config::http_server as cmp_http_server;
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # // insert from tests::fn_input_json
    /// # cmp_http_server::Config::<ExampleMessage> {
    /// #     port: 8000,
    /// fn_input: |msg: &Message<ExampleMessage>| {
    ///     let text = msg.serialize()?;
    ///     Ok(Some(text))
    /// },
    /// #    fn_output: |_| Ok(None),
    /// # };
    /// ```
    pub fn_input: fn(&Message<TMsg>) -> anyhow::Result<Option<String>>,

    /// Функция преобразования текста в сообщения
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// # use rsiot_components_config::http_server as cmp_http_server;
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # // insert from tests::stub
    /// # cmp_http_server::Config::<ExampleMessage> {
    /// #     port: 8000,
    /// #     fn_input: |_| Ok(None),
    /// fn_output: |_| Ok(None),
    /// # };
    /// ```
    ///
    /// ## Десериализация из json
    ///
    /// ```rust
    /// # use rsiot_components_config::http_server as cmp_http_server;
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # // insert from tests::fn_input_json
    /// # cmp_http_server::Config::<ExampleMessage> {
    /// #     port: 8000,
    /// #     fn_input: |_| Ok(None),
    /// fn_output: |text: &str| {
    ///     let msg = Message::deserialize(text)?;
    ///     Ok(Some(msg))
    /// },
    /// # };
    /// ```
    pub fn_output: fn(&str) -> anyhow::Result<Option<Message<TMsg>>>,
}

#[cfg(test)]
mod tests {
    use crate::http_server as cmp_http_server;
    use rsiot_messages_core::{example_message::*, *};

    #[allow(clippy::no_effect)]
    #[test]
    fn stub() {
        cmp_http_server::Config::<Custom> {
            port: 8000,
            fn_input: |_| Ok(None),
            fn_output: |_| Ok(None),
        };
    }

    #[allow(clippy::no_effect)]
    #[test]
    fn fn_input_json() {
        cmp_http_server::Config::<Custom> {
            port: 8000,
            fn_input: |msg: &Message<Custom>| {
                let text = msg.serialize()?;
                Ok(Some(text))
            },
            fn_output: |_| Ok(None),
        };
    }

    #[allow(clippy::no_effect)]
    #[test]
    fn fn_output_json() {
        cmp_http_server::Config::<Custom> {
            port: 8000,
            fn_input: |_| Ok(None),
            fn_output: |text: &str| {
                let msg = Message::deserialize(text)?;
                Ok(Some(msg))
            },
        };
    }
}
