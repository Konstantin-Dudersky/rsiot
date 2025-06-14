//! Конфигурация websocket-клиента

use crate::{
    components_config::websocket_general::WebsocketMessage, message::Message,
    serde_utils::SerdeAlgKind,
};

/// Преобразование входящих сообщений в текст для отправки на сервер
pub type FnClientToServer<TMsg, TClientToServer> = fn(&TMsg) -> Option<TClientToServer>;

/// Преобразование полученного от сервера текста в исходящие сообщения
pub type FnServerToClient<TMsg, TServerToClient> = fn(TServerToClient) -> Vec<TMsg>;

/// Настройки Websocket-клиента
#[derive(Clone, Debug)]
pub struct Config<TMsg, TServerToClient, TClientToServer>
where
    TServerToClient: WebsocketMessage,
    TClientToServer: WebsocketMessage,
{
    /// Алгоритм сериализации / десериализации
    pub serde_alg: SerdeAlgKind,

    /// Адрес Websocket-сервера
    /// "ws://localhost:9001"
    pub url: String,

    /// Преобразование входящих сообщений в текст для отправки на сервер
    pub fn_client_to_server: FnClientToServer<TMsg, TClientToServer>,

    /// Преобразование полученного от сервера текста в исходящие сообщения
    pub fn_server_to_client: FnServerToClient<TMsg, TServerToClient>,

    /// Функция создает исходящее сообщение с информацией о соединении
    ///
    /// В функцию передается состояние соединения;
    /// - true - соединение установлено
    /// - false - соединение разорвано
    ///
    /// Примеры см. в тестах
    pub fn_connection_state: fn(bool) -> Option<Message<TMsg>>,
}

impl<TMsg, TServerToClient, TClientToServer> Default
    for Config<TMsg, TServerToClient, TClientToServer>
where
    TServerToClient: WebsocketMessage,
    TClientToServer: WebsocketMessage,
{
    fn default() -> Self {
        Self {
            serde_alg: SerdeAlgKind::Json,
            url: "ws://localhost:8000".into(),
            fn_client_to_server: |_| None,
            fn_server_to_client: |_| vec![],
            fn_connection_state: |_| None,
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, clippy::field_reassign_with_default)]
mod tests {
    use serde::{Deserialize, Serialize};
    use strum::IntoStaticStr;

    use super::*;
    use crate::message::example_message::*;

    #[test]
    fn fn_connection_state() {
        // Заглушка
        let fn_connection_state_1 = |_| None;

        #[derive(Clone, Debug, Deserialize, IntoStaticStr, Serialize)]
        enum ServerToClient {}
        impl WebsocketMessage for ServerToClient {}

        #[derive(Clone, Debug, Deserialize, IntoStaticStr, Serialize)]
        enum ClientToServer {}
        impl WebsocketMessage for ClientToServer {}

        let mut config_1: Config<Custom, ServerToClient, ClientToServer> = Default::default();
        config_1.fn_connection_state = fn_connection_state_1;
    }
}
