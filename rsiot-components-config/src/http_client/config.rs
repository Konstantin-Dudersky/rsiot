use super::{
    connection_config::ConnectionConfig, request_input::RequestInput,
    request_periodic::RequestPeriodic,
};

/// Параметры компонента http-client
#[derive(Clone, Debug)]
pub struct Config<TMessage> {
    /// Параметры подключения
    pub connection_config: ConnectionConfig,
    /// Запросы, которые формируются на основе входящих сообщений
    pub requests_input: Vec<RequestInput<TMessage>>,
    /// Периодические запросы
    pub requests_periodic: Vec<RequestPeriodic<TMessage>>,
}
