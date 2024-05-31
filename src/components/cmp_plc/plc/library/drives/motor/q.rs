use serde::{Deserialize, Serialize};

/// Область памяти output
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    /// Статус для вывода на hmi
    pub status: QHmiStatus,

    /// 1 = команда на запуск двигателя
    pub start: bool,
}

/// Статут для вывода на hmi
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QHmiStatus {
    /// Активирован ручной режим
    pub man_act: bool,
    /// Активирован автоматический режим
    pub aut_act: bool,
    /// Разрешения для работы с hmi
    pub hmi_permission: QHmiPermission,
}

/// Разрешения для работы с hmi
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QHmiPermission {
    /// Разрешение включения в ручном режиме
    pub man_start: bool,
    /// Разрешение отключения в ручном режиме
    pub man_stop: bool,
}
