use std::time::Duration;

pub struct Buffer {
    pub last_value: f64,
    pub window: Duration,
}
