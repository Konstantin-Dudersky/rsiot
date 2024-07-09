use serde::{Deserialize, Serialize};

pub use super::super::select_mode::QMode;

/// Область памяти output
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    /// Статус для вывода на hmi
    pub hmi_status: QHmiStatus,

    /// Активно задание из plc
    pub mv_plc_act: bool,
    /// Активно задание из hmi
    pub mv_hmi_act: bool,
    /// Задание
    pub mv: f64,
}

/// Статут для вывода на hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct QHmiStatus {
    /// Состояние
    pub state: QState,

    /// Режим работы
    pub mode: QMode,

    /// Активно задание из plc
    pub mv_plc_act: bool,
    /// Активно задание из hmi
    pub mv_hmi_act: bool,
    /// Задание
    pub mv: f64,

    /// Фактическое открытие
    pub rbk: f64,

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

    /// Разрешение переключения в режим auto
    pub mode_auto: bool,
    /// Разрешение переключения в режим manual
    pub mode_man: bool,
    /// Разрешение переключения в режим local
    pub mode_local: bool,
    /// Разрешение переключения в режим oos
    pub mode_oos: bool,

    /// Активировать задание из hmi
    pub mv_hmi_en: bool,
    /// Активировать задание из plc
    pub mv_plc_en: bool,
    /// Активация любого задания: `sp_hmi_en && sp_plc_en`
    pub mv_hmi_plc_en: bool,
    /// Изменение значения sp
    pub mv_hmi: bool,
}

/// Состояние
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum QState {
    /// Закрыт
    #[default]
    Closed,

    /// Открытие
    Opening,

    /// Открыт
    Opened,

    /// Закрытие
    Closing,

    /// В ошибке
    Alarm,
}
