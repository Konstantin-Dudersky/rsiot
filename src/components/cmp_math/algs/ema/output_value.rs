use time::OffsetDateTime;

// ANCHOR: OutputValue
pub struct OutputValue {
    /// Значение экспоненциального скользящего среднего
    pub ema: f64,

    /// Метка времени
    pub time: OffsetDateTime,
}
// ANCHOR: OutputValue
