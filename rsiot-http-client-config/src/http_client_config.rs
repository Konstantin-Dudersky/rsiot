use rsiot_messages_core::IMessage;

use crate::{
    connection_config::ConnectionConfig, request_cyclic::RequestCyclic,
    request_on_event::RequestOnEvent,
};

pub struct HttpClientConfig<TMessage>
where
    TMessage: IMessage,
{
    /// Параметры подключения
    pub connection_config: ConnectionConfig,
    /// Запросы, которые формируются на основе входящих сообщений
    pub requests_on_event: Vec<RequestOnEvent>,
    /// Периодические запросы
    pub requests_cyclic: Vec<RequestCyclic<TMessage>>,
}
