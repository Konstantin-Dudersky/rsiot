use serde::{Deserialize, Serialize};

pub use super::super::select_mode::QMode;

/// Область памяти output
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    /// Статус для вывода на hmi
    pub hmi_status: QHmiStatus,

    /// 1 = команда на запуск двигателя
    pub start: bool,
}

/// Статут для вывода на hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QHmiStatus {
    /// Состояние
    pub state: QState,

    /// Режим работы
    pub mode: QMode,

    /// Разрешения для работы с hmi
    pub hmi_permission: QHmiPermission,

    /// true - команда на запуск
    pub start: bool,
}

/// Разрешения для работы с hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QHmiPermission {
    /// Разрешение включения в ручном режиме
    pub man_start: bool,
    /// Разрешение отключения в ручном режиме
    pub man_stop: bool,

    /// Разрешение переключения в режим auto
    pub mode_auto: bool,
    /// Разрешение переключения в режим manual
    pub mode_man: bool,
    /// Разрешение переключения в режим local
    pub mode_local: bool,
    /// Разрешение переключения в режим oos
    pub mode_oos: bool,
}

/// Состояние
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum QState {
    /// Стоп
    #[default]
    Stop,

    /// Старт
    Start,

    /// В ошибке
    Alarm,
}
