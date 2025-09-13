use std::time::Duration;

use time::OffsetDateTime;

// ANCHOR: OutputValue
pub struct OutputValue {
    /// Значение дифференциала
    pub differential: f64,

    /// Метка времени
    pub time: OffsetDateTime,

    pub time_window: Duration,
}
// ANCHOR: OutputValue
