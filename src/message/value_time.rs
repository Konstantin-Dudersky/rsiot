use time::OffsetDateTime;

/// Значение с меткой времени
#[derive(Clone, Copy, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
#[cfg_attr(feature = "cmp_timescaledb", derive(sqlx::FromRow))]
pub struct ValueTime {
    /// Значение
    pub value: f64,

    /// Метка времени
    pub time: OffsetDateTime,
}

impl Default for ValueTime {
    fn default() -> Self {
        Self {
            value: Default::default(),
            time: OffsetDateTime::now_utc(),
        }
    }
}
