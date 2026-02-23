use std::time::Duration;

pub enum IntMsg {
    Value {
        elapsed: Duration,
        count: u32,
        frequency: f64,
    },

    Tick(),
}
