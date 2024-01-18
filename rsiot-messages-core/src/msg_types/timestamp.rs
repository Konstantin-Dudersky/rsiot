use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

/// Метка времени
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Timestamp(pub DateTime<FixedOffset>);

impl Default for Timestamp {
    fn default() -> Self {
        Self(Utc::now().into())
    }
}
