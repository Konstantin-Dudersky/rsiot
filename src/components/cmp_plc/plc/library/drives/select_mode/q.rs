use serde::{Deserialize, Serialize};

/// Область памяти output
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    /// Режим работы
    pub mode: QMode,

    /// Статус для вывода на hmi
    pub hmi_status: QHmiStatus,
}

/// Статут для вывода на hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QHmiStatus {
    /// Режим работы
    pub mode: QMode,
    /// Разрешения для работы с hmi
    pub hmi_permission: QHmiPermission,
}

/// Разрешения для работы с hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QHmiPermission {
    /// Разрешение переключения в режим auto
    pub mode_auto: bool,
    /// Разрешение переключения в режим manual
    pub mode_man: bool,
    /// Разрешение переключения в режим local
    pub mode_local: bool,
    /// Разрешение переключения в режим oos
    pub mode_oos: bool,
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
