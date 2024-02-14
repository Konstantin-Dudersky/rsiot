use rsiot_messages_core::msg_meta::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ping {
    pub counter: u32,
    pub source_ts: Timestamp,
}

impl Ping {
    pub fn new() -> Self {
        Self {
            counter: 0,
            source_ts: Timestamp::default(),
        }
    }

    pub fn ping(&mut self) -> Self {
        self.counter += 1;
        self.source_ts = Timestamp::default();
        self.clone()
    }
}
