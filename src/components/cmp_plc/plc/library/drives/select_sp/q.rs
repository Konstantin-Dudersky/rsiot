use serde::{Deserialize, Serialize};

/// Область памяти output
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    /// Активно задание из plc
    pub sp_plc_act: bool,
    /// Активно задание из hmi
    pub sp_hmi_act: bool,
    /// Задание
    pub sp: f64,

    /// Статус для вывода на hmi
    pub hmi_status: QHmiStatus,
}

/// Статут для вывода на hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QHmiStatus {
    /// Активно задание из plc
    pub sp_plc_act: bool,
    /// Активно задание из hmi
    pub sp_hmi_act: bool,
    /// Задание
    pub sp: f64,

    /// Разрешения для работы с hmi
    pub hmi_permission: QHmiPermission,
}

/// Разрешения для работы с hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QHmiPermission {
    /// Активировать задание из hmi
    pub sp_hmi_en: bool,
    /// Активировать задание из plc
    pub sp_plc_en: bool,
    /// Активация любого задания: `sp_hmi_en && sp_plc_en`
    pub sp_hmi_plc_en: bool,
    /// Изменение значения sp
    pub sp_hmi: bool,
}
