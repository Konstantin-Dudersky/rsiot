use std::time::Duration;

use super::{
    http_param::HttpParam,
    types::{CbkOnFailure, CbkOnSuccess},
};

/// Параметры периодического запроса
#[derive(Clone, Debug)]
pub struct RequestPeriodic<TMessage> {
    /// Периодичность вызова
    pub period: Duration,
    /// Параметры запроса
    pub http_param: HttpParam,
    /// Функция обработки корректного ответа
    pub on_success: CbkOnSuccess<TMessage>,
    /// Функция обработки некорректного ответа
    pub on_failure: CbkOnFailure<TMessage>,
}
