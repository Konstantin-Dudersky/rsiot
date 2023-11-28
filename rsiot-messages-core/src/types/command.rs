use serde::{Deserialize, Serialize};

use chrono::{DateTime, FixedOffset, Utc};

#[derive(Serialize, Clone, Deserialize, Debug, Copy, PartialEq)]
pub struct Command {
    pub ts: DateTime<FixedOffset>,
}

impl Command {
    pub fn new(ts: Option<DateTime<FixedOffset>>) -> Self {
        let ts = match ts {
            Some(value) => value,
            None => Utc::now().into(),
        };
        Self { ts }
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            ts: Utc::now().into(),
        }
    }
}
