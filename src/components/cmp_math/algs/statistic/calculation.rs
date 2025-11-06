use std::{cmp::Ordering, collections::VecDeque, time::Duration};

use crate::message::ValueTime;

use super::{Indicators, OutputValue};

pub struct Calculation {
    time_window: Duration,
    indicators: Indicators,
    buffer: VecDeque<ValueTime>,
}

impl Calculation {
    pub fn new(time_window: Duration, indicators: Indicators) -> Self {
        Self {
            time_window,
            indicators,
            buffer: VecDeque::new(),
        }
    }

    pub fn step(&mut self, iv: ValueTime) -> Option<OutputValue> {
        let start_time = iv.time - self.time_window;
        let mut ov = OutputValue {
            value: iv.value,
            time: iv.time,
            max: None,
            min: None,
            range: None,
            time_window: self.time_window,
        };

        self.buffer.push_back(iv);

        loop {
            let Some(item) = self.buffer.front() else {
                break;
            };

            if item.time < start_time {
                self.buffer.pop_front();
            } else {
                break;
            }
        }

        ov.max = match self.indicators.max || self.indicators.range {
            true => self
                .buffer
                .iter()
                .max_by(|x, y| x.value.partial_cmp(&y.value).unwrap_or(Ordering::Equal))
                .cloned(),
            false => None,
        };

        ov.min = match self.indicators.min || self.indicators.range {
            true => self
                .buffer
                .iter()
                .min_by(|x, y| x.value.partial_cmp(&y.value).unwrap_or(Ordering::Equal))
                .cloned(),
            false => None,
        };

        ov.range = match self.indicators.range {
            true => match (ov.max, ov.min) {
                (Some(max), Some(min)) => Some(max.value - min.value),
                _ => None,
            },
            false => None,
        };

        Some(ov)
    }
}
