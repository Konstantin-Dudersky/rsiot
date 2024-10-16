use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S<TState> {
    /// Текущее состояние
    pub current_state: TState,
    /// Время нахождения в состоянии
    pub state_time: Duration,
}
