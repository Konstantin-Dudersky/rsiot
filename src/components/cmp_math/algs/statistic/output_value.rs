use std::time::Duration;

use time::OffsetDateTime;

use crate::message::ValueTime;

// ANCHOR: OutputValue
/// Выходное значение алгоритма
pub struct OutputValue {
    /// Значение
    pub value: f64,

    /// Метка времени
    pub time: OffsetDateTime,

    /// Найденное максимальное значение
    pub max: Option<ValueTime>,

    /// Найденное минимальное значение
    pub min: Option<ValueTime>,

    /// Разница между максимальным и минимальным значениями
    pub range: Option<f64>,

    /// Окно времени
    pub time_window: Duration,
}
// ANCHOR: OutputValue
