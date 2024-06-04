use serde::{Deserialize, Serialize};

use super::{QMode, QState};

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    /// режим работы
    pub mode: QMode,

    /// Команда на запуск
    pub state: QState,
}
