use std::time::Instant;

use tracing::warn;

pub struct MeasureExecutionTime<const SIZE: usize> {
    values: [u128; SIZE],
    current_index: usize,
    start_time: Option<Instant>,
    buffer_full: bool,
}
impl<const SIZE: usize> MeasureExecutionTime<SIZE> {
    pub fn new() -> Self {
        Self {
            values: [0; SIZE],
            current_index: 0,
            start_time: None,
            buffer_full: false,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        let Some(start_time) = self.start_time else {
            warn!("Call stop without start");
            return;
        };
        let elapsed = start_time.elapsed().as_nanos();
        self.start_time = None;

        self.values[self.current_index] = elapsed;

        self.current_index += 1;
        if self.current_index >= SIZE {
            self.buffer_full = true;
            self.current_index = 0;
        }
    }

    pub fn stat(&self) -> String {
        let slice_end = match self.buffer_full {
            true => SIZE - 1,
            false => self.current_index - 1,
        };

        let max = self
            .values
            .into_iter()
            .take(slice_end)
            .max()
            .unwrap_or(u128::MAX);

        let min = self
            .values
            .into_iter()
            .take(slice_end)
            .min()
            .unwrap_or(u128::MIN);

        let sum: u128 = self.values.into_iter().sum();
        let avg = sum as f64 / slice_end as f64;

        format!("AVG: {:.0}, MIN: {}, MAX: {}", avg, min, max)
    }
}
