use crate::message::Message;

use super::WebsocketMessage;

/// Конфигурация Websocket-сервера
#[derive(Clone, Debug)]
pub struct Config<TMsg, TServerToClient, TClientToServer>
where
    TServerToClient: WebsocketMessage,
    TClientToServer: WebsocketMessage,
{
    /// Порт, через который доступен сервер
    pub port: u16,

    /// Функция преобразования входящих сообщений в перечисление, пересылаемое по вебсокету
    pub fn_input: FnInput<TMsg, TServerToClient>,

    /// Функция перечисления, пересылаемых по вебсокету, в исходящий поток сообщений
    pub fn_output: FnOutput<TMsg, TClientToServer>,
}

/// Функция преобразования входящих сообщений в перечисление, пересылаемое по вебсокету
pub type FnInput<TMsg, TServerToClient> = fn(&Message<TMsg>) -> Option<TServerToClient>;
/// Функция перечисления, пересылаемых по вебсокету, в исходящий поток сообщений
pub type FnOutput<TMsg, TClientToServer> = fn(TClientToServer) -> Vec<Message<TMsg>>;
