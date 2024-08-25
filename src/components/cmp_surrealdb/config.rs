use surrealdb::Response;

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
    pub request_input: Vec<RequestInputConfig<TMsg>>,

    /// Конфигурация запросов, выполняющихся при запуске
    pub request_start: Vec<RequestStartConfig<TMsg>>,
}

pub type FnOnSuccess<TMessage> = fn(Response) -> Result<Vec<Message<TMessage>>, anyhow::Error>;
pub type FnOnFailure<TMessage> = fn() -> Vec<Message<TMessage>>;

/// Конфигурация запросов, которые выполняются на основе входного потока сообщений
#[derive(Clone, Debug)]
pub struct RequestInputConfig<TMsg> {
    /// Функция формирования запроса на основе потока сообщений
    pub fn_input: fn(&Message<TMsg>) -> Option<String>,
    /// Функция вызывается при успешно выполненном запросе
    pub fn_on_success: FnOnSuccess<TMsg>,
    /// Функция вызывается при ошибке выполнения запроса
    pub fn_on_failure: FnOnFailure<TMsg>,
}

/// Конфигурация запросов, которые выполняются один раз при запуске
#[derive(Clone, Debug)]
pub struct RequestStartConfig<TMsg> {
    /// Функция формирования запроса на основе потока сообщений
    pub query: String,
    /// Функция вызывается при успешно выполненном запросе
    pub fn_on_success: FnOnSuccess<TMsg>,
    /// Функция вызывается при ошибке выполнения запроса
    pub fn_on_failure: FnOnFailure<TMsg>,
}
