//! Конфигурация запросов, которые выполняются на основе входного потока сообщений

use super::{FnOnFailure, FnOnSuccess, Request, Response};

/// Конфигурация запросов, которые выполняются на основе входного потока сообщений
#[derive(Clone, Debug)]
pub struct InputConfig<TMessage> {
    /// Функция формирования запроса на основе потока сообщений
    pub fn_input: fn(&TMessage) -> Option<Request>,
    /// Функция вызывается при успешно выполненном запросе
    pub fn_on_success: FnOnSuccess<TMessage>,
    /// Функция вызывается при ошибке выполнения запроса
    pub fn_on_failure: FnOnFailure<TMessage>,
}
