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
}

/// TODO - вместо Utc использовать местный часовой пояс?
impl Default for Timestamp {
    fn default() -> Self {
        Self(Utc::now().into())
    }
}
