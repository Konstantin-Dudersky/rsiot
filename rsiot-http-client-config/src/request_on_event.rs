use crate::{
    types::{CbkOnFailure, CbkOnSuccess},
    RequestParam,
};

#[derive(Clone)]
pub struct RequestOnEvent<TMessage> {
    /// Функция выдает параметры запроса, на основе входных сообщений
    pub condition: fn(TMessage) -> Option<RequestParam>,
    /// Функция обработки корректного ответа
    pub on_success: CbkOnSuccess<TMessage>,
    /// Функция обработки некорректного ответа
    pub on_failure: CbkOnFailure<TMessage>,
}
