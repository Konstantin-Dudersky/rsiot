use crate::message::Message;

/// Конфигурация cmp_surrealdb
#[derive(Clone, Debug)]
pub struct Config<TMsg> {
    /// localhost
    pub host: String,

    /// 8000
    pub port: u16,

    /// root
    pub user: String,

    /// root
    pub password: String,

    /// rsiot
    pub namespace: String,

    /// rsiot
    pub database: String,

    /// Скрипт для инициализации
    pub init_script: String,

    /// Конфигурация запросов на основе входных сообщений
    pub request_input: Vec<InputConfig<TMsg>>,
}

pub type FnOnSuccess<TMessage> = fn(&String) -> Vec<Message<TMessage>>;
pub type FnOnFailure<TMessage> = fn() -> Vec<Message<TMessage>>;

/// Конфигурация запросов, которые выполняются на основе входного потока сообщений
#[derive(Clone, Debug)]
pub struct InputConfig<TMsg> {
    /// Функция формирования запроса на основе потока сообщений
    pub fn_input: fn(&Message<TMsg>) -> Option<String>,
    /// Функция вызывается при успешно выполненном запросе
    pub fn_on_success: FnOnSuccess<TMsg>,
    /// Функция вызывается при ошибке выполнения запроса
    pub fn_on_failure: FnOnFailure<TMsg>,
}
