use serde::{Deserialize, Serialize};

use super::super::{mode_select, sp_plc_hmi};

use super::QState;

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    /// Режим работы
    pub mode: mode_select::FB,

    /// Команда на запуск
    pub state: QState,

    /// Задание открытия
    pub mv: sp_plc_hmi::FB,
}
