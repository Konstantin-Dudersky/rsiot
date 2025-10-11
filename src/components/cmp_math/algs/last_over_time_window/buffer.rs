use std::time::Duration;

use crate::message::ValueTime;

pub struct Buffer {
    pub last_value: ValueTime,
    pub window: Duration,
}
