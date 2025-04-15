use serde::{Deserialize, Serialize};

/// Входная структура
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct I<TState> {
    /// Новое состояние
    pub new_state: TState,
}
