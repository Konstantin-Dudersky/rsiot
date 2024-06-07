use serde::{Deserialize, Serialize};

/// Входная структура
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    /// Источник выбора режима: 0 = из plc, 1 = из hmi
    pub mode_plc_hmi: bool,
    /// Переключение в режим auto из контроллера
    pub auto_mode_plc: bool,
    /// Переключение в режим man из контроллера
    pub man_mode_plc: bool,

    /// Выбор источника команд выбора (`sp_hmi_en`, `sp_plc_en`):
    /// false => из hmi
    /// true => из plc
    pub mv_en_select: bool,
    /// Активировать задание из hmi
    pub mv_hmi_en: bool,
    /// Активировать задание из plc
    pub mv_plc_en: bool,
    /// Задание из plc
    pub mv_plc: f64,

    /// Команда с hmi
    pub hmi_command: IHmiCommand,
}

/// Команда с hmi
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum IHmiCommand {
    /// Нет команды - по-умолчанию
    #[default]
    NoCommand,

    /// Переключение в режим man из hmi
    ManMode,
    /// Переключение в режим auto из hmi
    AutoMode,
    /// Переключение в режим local из hmi
    LocalMode,
    /// Переключение в режим oos из hmi
    OosMode,

    /// Активировать задание из hmi
    mv_hmi_en,
    /// Активировать задание из plc
    mv_plc_en,
    /// Задание из hmi
    mv_hmi(f64),
}
