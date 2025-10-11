use std::time::Duration;

use crate::message::ValueTime;

use super::OutputValue;

pub struct Calculation {
    time_window: Duration,
    last_value: Option<ValueTime>,
}

impl Calculation {
    pub fn new(time_window: Duration) -> Self {
        Self {
            time_window,
            last_value: None,
        }
    }

    pub fn step(&mut self, iv: ValueTime) -> Option<OutputValue> {
        let Some(last_value) = &self.last_value else {
            return self.update_last_value(iv);
        };

        if (last_value.time - iv.time).abs() > self.time_window {
            return self.update_last_value(iv);
        }

        None
    }

    fn update_last_value(&mut self, iv: ValueTime) -> Option<OutputValue> {
        self.last_value = Some(iv);
        Some(OutputValue {
            value: iv.value,
            timestamp: iv.time,
            time_window: self.time_window,
        })
    }
}
