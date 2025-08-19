use crate::{message::Message, serde_utils::SerdeAlgKind};

use super::WebsocketMessage;

// ANCHOR: Config
/// Конфигурация Websocket-сервера
#[derive(Clone, Debug)]
pub struct Config<TMsg, TServerToClient, TClientToServer>
where
    TServerToClient: WebsocketMessage,
    TClientToServer: WebsocketMessage,
{
    /// Алгоритм сериализации и десериализации сообщений
    pub serde_alg: SerdeAlgKind,

    /// Порт, через который доступен сервер
    pub port: u16,

    /// Функция преобразования входящих сообщений в перечисление, пересылаемое по вебсокету
    pub fn_server_to_client: FnInput<TMsg, TServerToClient>,

    /// Функция перечисления, пересылаемых по вебсокету, в исходящий поток сообщений
    pub fn_client_to_server: FnOutput<TMsg, TClientToServer>,
}
// ANCHOR: Config

/// Функция преобразования входящих сообщений в перечисление, пересылаемое по вебсокету
pub type FnInput<TMsg, TServerToClient> = fn(&Message<TMsg>) -> Option<TServerToClient>;
/// Функция перечисления, пересылаемых по вебсокету, в исходящий поток сообщений
pub type FnOutput<TMsg, TClientToServer> = fn(TClientToServer) -> Vec<Message<TMsg>>;
