use std::time::Duration;

use time::OffsetDateTime;

// ANCHOR: OutputValue
/// Выходное значение алгоритма
pub struct OutputValue {
    /// Значение
    pub value: f64,

    /// Метка времени
    pub time: OffsetDateTime,

    /// Окно времени
    pub time_window: Duration,
}
// ANCHOR: OutputValue
