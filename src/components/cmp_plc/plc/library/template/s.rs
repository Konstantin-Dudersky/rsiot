use serde::{Deserialize, Serialize};

use super::QState;

use super::super::drives::mode_select;

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    /// Режим работы
    pub mode: mode_select::FB,

    /// Команда на запуск
    pub state: QState,
}
