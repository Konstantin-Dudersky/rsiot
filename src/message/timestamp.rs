use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike, Weekday};
use serde::{Deserialize, Serialize};

/// Метка времени
///
/// Тип на основе `chrono::DateTime<FixedOffset>`. По-умолчанию создается текущая метка времени.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Timestamp(pub DateTime<FixedOffset>);

impl Timestamp {
    /// Преобразовать в строку с заданным форматом
    pub fn format(&self, fmt: &str) -> String {
        self.0.format(fmt).to_string()
    }

    /// Returns an RFC 3339 and ISO 8601 date and time string such as `1996-12-19T16:39:57-08:00`.
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    /// Возвращает время Unix с наносекундной точностью
    pub fn timestamp_nanos_opt(&self) -> Option<i64> {
        self.0.timestamp_nanos_opt()
    }

    /// Возвращает номер дня недели. 1 = понедельник, 7 = воскресенье
    pub fn weekday(&self) -> u8 {
        match self.0.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 7,
        }
    }

    /// Часы
    pub fn hour(&self) -> u32 {
        self.0.hour()
    }

    /// Минуты
    pub fn minute(&self) -> u32 {
        self.0.minute()
    }

    /// Секунды
    pub fn second(&self) -> u32 {
        self.0.second()
    }
}

/// TODO - вместо Utc использовать местный часовой пояс?
impl Default for Timestamp {
    fn default() -> Self {
        Self(Local::now().into())
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
