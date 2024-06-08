use serde::{Deserialize, Serialize};

use super::QState;

use super::super::drives::select_mode;

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    /// Режим работы
    pub mode: select_mode::FB,

    /// Команда на запуск
    pub state: QState,
}
