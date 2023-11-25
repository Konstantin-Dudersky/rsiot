use rsiot_messages_core::IMessage;

use super::{
    connection_config::ConnectionConfig, request_on_event::RequestOnEvent,
    request_periodic::RequestPeriodic,
};

pub struct HttpClientConfig<TMessage>
where
    TMessage: IMessage,
{
    /// Параметры подключения
    pub connection_config: ConnectionConfig,
    /// Запросы, которые формируются на основе входящих сообщений
    pub requests_on_event: Vec<RequestOnEvent<TMessage>>,
    /// Периодические запросы
    pub requests_periodic: Vec<RequestPeriodic<TMessage>>,
}
