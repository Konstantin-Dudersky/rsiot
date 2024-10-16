use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Область памяти output
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q<TState> {
    /// Текущее состояние
    pub current_state: TState,
    /// Время нахождения в состоянии
    pub state_time: Duration,
    /// Первое выполнение в новом цикле
    pub is_first_cycle: bool,
}
