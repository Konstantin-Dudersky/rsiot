use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Входная структура
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct I<TState> {
    /// Новое состояние
    pub new_state: TState,

    /// Время цикла между вызовами блока
    pub cycle_time: Duration,
}
