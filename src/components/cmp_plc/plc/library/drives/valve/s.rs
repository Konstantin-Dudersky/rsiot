use serde::{Deserialize, Serialize};

use super::super::select_mode;

/// Область памяти stat
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S {
    /// Режим работы
    pub mode: select_mode::FB,

    /// Команда на запуск
    pub control: bool,
}
