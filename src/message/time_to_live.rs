//! Ограничение времени жизни сообщения

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Ограничение времени жизни сообщения
///
/// # Примеры
///
/// Все сообщения без ограничения по времени
///
/// ```rust
/// impl TimeToLive for Custom {
///     fn time_to_live(&self) -> TimeToLiveValue {
///         TimeToLiveValue::Infinite
///     }
/// }
/// ```
pub trait TimeToLive {
    /// Ограничение времени жизни сообщения
    fn time_to_live(&self) -> TimeToLiveValue {
        TimeToLiveValue::Infinite
    }
}

/// Значение ограничения времени жизни сообщения
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TimeToLiveValue {
    /// Без ограничения
    Infinite,
    /// Заданное время
    Duration(Duration),
    /// Запретить кеширование
    DisableCaching,
}
