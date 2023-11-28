use super::{
    types::{CbkOnFailure, CbkOnSuccess},
    HttpParam,
};

/// Параметры запроса на основе входящего потока сообщений
#[derive(Clone, Debug)]
pub struct RequestInput<TMessage> {
    /// Функция выдает параметры запроса, на основе входных сообщений
    pub fn_input: fn(&TMessage) -> Option<HttpParam>,
    /// Функция обработки корректного ответа
    pub on_success: CbkOnSuccess<TMessage>,
    /// Функция обработки некорректного ответа
    pub on_failure: CbkOnFailure<TMessage>,
}
