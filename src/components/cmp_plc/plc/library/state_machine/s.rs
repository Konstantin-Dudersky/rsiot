use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Область памяти stat
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S<TState> {
    /// Текущее состояние
    pub current_state: TState,
    /// Время нахождения в состоянии
    pub state_time: Duration,
}
