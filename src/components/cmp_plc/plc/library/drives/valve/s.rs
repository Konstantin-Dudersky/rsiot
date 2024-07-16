use serde::{Deserialize, Serialize};

use super::super::select_mode;

use super::QState;

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    /// Режим работы
    pub mode: select_mode::FB,

    /// Команда на запуск
    pub control: bool,
}
