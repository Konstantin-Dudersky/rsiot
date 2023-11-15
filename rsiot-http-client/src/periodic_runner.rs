//! Периодическое выполнение запросов

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct PeriodicRunner {
    period: Duration,
    last_run: Option<Instant>,
}

impl PeriodicRunner {
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            last_run: None,
        }
    }

    pub fn check(&mut self) -> bool {
        let last_run = match self.last_run {
            Some(val) => val,
            None => {
                self.last_run = Some(Instant::now());
                return true;
            }
        };
        if last_run.elapsed() >= self.period {
            self.last_run = Some(Instant::now());
            return true;
        }
        false
    }
}
