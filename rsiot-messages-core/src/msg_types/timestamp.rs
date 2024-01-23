use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

/// Метка времени
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Timestamp(pub DateTime<FixedOffset>);

impl Timestamp {
    pub fn format(&self, fmt: &str) -> String {
        self.0.format(fmt).to_string()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self(Utc::now().into())
    }
}
