use std::time::Duration;

use instant::Instant;
use serde::{Deserialize, Serialize};

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S<TState> {
    /// Текущее состояние
    pub current_state: TState,
    /// Время нахождения в состоянии
    pub state_time: Duration,
    pub last_call: Instant,
}
