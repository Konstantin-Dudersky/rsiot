//! Ограничение времени жизни сообщения

use std::time::Duration;

use serde::{Deserialize, Serialize};

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
