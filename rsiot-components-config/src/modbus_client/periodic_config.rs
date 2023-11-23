//! Конфигурация запросов, которые выполняются периодически

use std::time::Duration;

use super::{FnOnFailure, FnOnSuccess, Request};

/// Конфигурация запросов, которые выполняются периодически
#[derive(Clone, Debug)]
pub struct PeriodicConfig<TMessage> {
    /// Периодичность выполенения запроса
    pub period: Duration,
    /// Запрос для выполнения
    pub request: Request,
    /// Функция вызывается при успешно выполненном запросе
    pub fn_on_success: FnOnSuccess<TMessage>,
    /// Функция вызывается при ошибке выполнения запроса
    pub fn_on_failure: FnOnFailure<TMessage>,
}
