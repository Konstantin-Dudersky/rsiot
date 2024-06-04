use serde::{Deserialize, Serialize};

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
}

/// Разрешения для работы с hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QHmiPermission {
    /// Разрешение включения в ручном режиме
    pub man_start: bool,
    /// Разрешение отключения в ручном режиме
    pub man_stop: bool,

    pub auto_mode: bool,

    pub man_mode: bool,

    pub local_mode: bool,

    pub oos_mode: bool,
}

/// Режим работы
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum QMode {
    /// Автоматический режим
    Auto,

    /// Местный
    Local,

    /// Ручной
    #[default]
    Manual,

    /// Out of service - выведен из эксплуатации
    Oos,
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
