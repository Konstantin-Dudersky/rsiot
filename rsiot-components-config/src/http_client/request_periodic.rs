use std::time::Duration;

use rsiot_messages_core::IMessage;

use super::{
    request_param::RequestParam,
    types::{CbkOnFailure, CbkOnSuccess},
};

#[derive(Clone)]
pub struct RequestPeriodic<TMessage>
where
    TMessage: IMessage,
{
    /// Периодичность вызова
    pub period: Duration,
    /// Параметры запроса
    pub request_param: RequestParam,
    /// Функция обработки корректного ответа
    pub on_success: CbkOnSuccess<TMessage>,
    /// Функция обработки некорректного ответа
    pub on_failure: CbkOnFailure<TMessage>,
}
