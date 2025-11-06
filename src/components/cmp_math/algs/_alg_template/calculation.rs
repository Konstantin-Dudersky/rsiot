use std::time::Duration;

use crate::message::ValueTime;

use super::OutputValue;

pub struct Calculation {
    time_window: Duration,
}

impl Calculation {
    pub fn new(time_window: Duration) -> Self {
        Self { time_window }
    }

    pub fn step(&mut self, _iv: ValueTime) -> Option<OutputValue> {
        None
    }
}
