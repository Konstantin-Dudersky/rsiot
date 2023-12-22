use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Config<TMessage> {
    /// Период опроса
    pub period: Duration,

    /// Функция преобразования значения с пинов в сообщения
    pub fn_output: fn(&bool) -> Vec<TMessage>,
}
