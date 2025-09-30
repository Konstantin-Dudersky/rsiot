use serde::{Deserialize, Serialize};
use time::{
    OffsetDateTime, Weekday,
    format_description::{self, well_known::Rfc3339},
};

/// Метка времени
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Timestamp(OffsetDateTime);

impl Timestamp {
    /// Преобразовать в строку с заданным форматом
    pub fn format(&self, fmt: &str) -> Result<String, String> {
        let format = format_description::parse(fmt).map_err(|e| e.to_string())?;
        self.0.format(&format).map_err(|e| e.to_string())
    }

    /// Returns an RFC 3339 and ISO 8601 date and time string such as `1996-12-19T16:39:57-08:00`.
    pub fn to_rfc3339(&self) -> Result<String, String> {
        self.0.format(&Rfc3339).map_err(|e| e.to_string())
    }

    /// Возвращает время Unix с наносекундной точностью
    pub fn unix_timestamp_nanos(&self) -> i128 {
        self.0.unix_timestamp_nanos()
    }

    /// Возвращает номер дня недели. 1 = понедельник, 7 = воскресенье
    pub fn weekday(&self) -> u8 {
        match self.0.weekday() {
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
            Weekday::Sunday => 7,
        }
    }

    /// Часы
    pub fn hour(&self) -> u8 {
        self.0.hour()
    }

    /// Минуты
    pub fn minute(&self) -> u8 {
        self.0.minute()
    }

    /// Секунды
    pub fn second(&self) -> u8 {
        self.0.second()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        let local_time = OffsetDateTime::now_local();
        let local_time = match local_time {
            Ok(v) => v,
            Err(_) => OffsetDateTime::now_utc(),
        };
        Self(local_time)
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
