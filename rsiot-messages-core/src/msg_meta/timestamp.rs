use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

/// Метка времени
///
/// Тип на основе `chrono::DateTime<FixedOffset>`. По-умолчанию создается текущая метка времени.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Timestamp(pub DateTime<FixedOffset>);

impl Timestamp {
    pub fn format(&self, fmt: &str) -> String {
        self.0.format(fmt).to_string()
    }

    /// Returns an RFC 3339 and ISO 8601 date and time string such as `1996-12-19T16:39:57-08:00`.
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn timestamp_nanos_opt(&self) -> Option<i64> {
        self.0.timestamp_nanos_opt()
    }
}

/// TODO - вместо Utc использовать местный часовой пояс?
impl Default for Timestamp {
    fn default() -> Self {
        Self(Utc::now().into())
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
