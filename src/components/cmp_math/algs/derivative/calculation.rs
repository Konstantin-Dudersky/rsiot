use std::{collections::VecDeque, time::Duration};

use crate::message::ValueTime;

use super::{Gamma, OutputValue};

pub struct Calculation {
    out_value: Option<ValueTime>,
    buffer: VecDeque<ValueTime>,
    gamma: Gamma,
    normalization_time: Duration,
}

impl Calculation {
    pub fn new(gamma: Gamma, normalization_time: Duration) -> Self {
        Self {
            out_value: None,
            buffer: VecDeque::new(),
            gamma,
            normalization_time,
        }
    }

    pub fn step(&mut self, new_value: ValueTime, time_window: Duration) -> OutputValue {
        self.buffer.push_back(new_value);
        let out_value = shrink_buffer(&mut self.buffer, time_window);

        self.out_value = match (self.out_value, out_value) {
            (None, None) => None,
            (None, Some(_)) => out_value,
            (Some(_), None) => self.out_value,
            (Some(old), Some(new)) => {
                if new.time >= old.time {
                    out_value
                } else {
                    self.out_value
                }
            }
        };

        let diff = match self.out_value {
            Some(out_value) => diff_with_out_value(
                &mut self.buffer,
                &out_value,
                self.gamma,
                self.normalization_time,
            ),
            None => diff_only_buffer(&mut self.buffer, self.gamma, self.normalization_time),
        };

        match diff {
            Some(v) => OutputValue {
                derivative: v.value,
                time: v.time,
                time_window,
                normalization_time: self.normalization_time,
            },
            None => OutputValue {
                derivative: 0.0,
                time: new_value.time,
                time_window,
                normalization_time: self.normalization_time,
            },
        }
    }
}

fn shrink_buffer(buffer: &mut VecDeque<ValueTime>, time_window: Duration) -> Option<ValueTime> {
    let mut out_value: Option<ValueTime> = None;
    let begin_ts = match buffer.back() {
        Some(v) => v.time - time_window,
        None => return out_value,
    };
    loop {
        let value = buffer.front();
        let Some(value) = value else { break };
        if value.time < begin_ts {
            out_value = buffer.pop_front();
        } else {
            break;
        }
    }
    out_value
}

fn diff_only_buffer(
    buffer: &mut VecDeque<ValueTime>,
    gamma: Gamma,
    normalization_time: Duration,
) -> Option<ValueTime> {
    if let Some(front) = buffer.front()
        && let Some(back) = buffer.back()
    {
        let diff = caclulate_derivative(front, back, gamma, normalization_time);
        return Some(diff);
    }
    None
}

fn diff_with_out_value(
    buffer: &mut VecDeque<ValueTime>,
    out_value: &ValueTime,
    gamma: Gamma,
    normalization_time: Duration,
) -> Option<ValueTime> {
    if let Some(back) = buffer.back() {
        let diff = caclulate_derivative(out_value, back, gamma, normalization_time);
        return Some(diff);
    }
    None
}

fn caclulate_derivative(
    front: &ValueTime,
    back: &ValueTime,
    gamma: Gamma,
    normalization_time: Duration,
) -> ValueTime {
    let diff_time = back.time - front.time;
    if diff_time == Duration::from_millis(0) {
        return ValueTime {
            value: 0.0,
            time: back.time,
        };
    }
    let diff_time = diff_time / normalization_time;

    let diff_value = back.value - front.value;
    let gamma: f64 = gamma.into();

    let diff = diff_value / diff_time.powf(gamma);

    ValueTime {
        value: diff,
        time: back.time,
    }
}
