use std::time::Duration;

use crate::message::Message;

use super::SystemInfo;

/// Конфигурация cmp_system_info
pub struct Config<TMsg> {
    /// Период обновления данных
    pub period: Duration,

    /// Функция создания исходящих сообщений на основе данных системы
    pub fn_output: fn(&SystemInfo) -> Vec<Message<TMsg>>,
}
