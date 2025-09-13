use std::time::Duration;

use time::OffsetDateTime;

// ANCHOR: OutputValue
pub struct OutputValue {
    /// Производная
    pub derivative: f64,

    /// Метка времени
    pub time: OffsetDateTime,

    /// Окно времени
    pub time_window: Duration,

    /// Время нормализации
    pub normalization_time: Duration,
}
// ANCHOR: OutputValue
