use std::time::Duration;

use crate::executor::Instant;

/// Выполнение каких-то действий реже, чем за заданный период времени.
pub struct LessInPeriod {
    last_exec: Option<Instant>,
    period: Duration,
}

impl LessInPeriod {
    pub fn new(period: Duration) -> Self {
        LessInPeriod {
            last_exec: None,
            period,
        }
    }

    pub fn check(&mut self) -> bool {
        let last_exec = match &self.last_exec {
            Some(v) => v,
            None => {
                self.last_exec = Some(Instant::now());
                return true;
            }
        };

        if last_exec.elapsed() >= self.period {
            self.last_exec = Some(Instant::now());
            return true;
        }

        false
    }
}
