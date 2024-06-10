use serde::{Deserialize, Serialize};

/// Входная структура
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    /// Выбор источника команд выбора (`sp_hmi_en`, `sp_plc_en`):
    /// false => из hmi
    /// true => из plc
    pub sp_en_source: bool,
    /// Активировать задание из hmi
    pub sp_hmi_en: bool,
    /// Активировать задание из plc
    pub sp_plc_en: bool,
    /// Задание из plc
    pub sp_plc: f64,

    /// Команда с hmi
    pub hmi_command: IHmiCommand,
}

/// Команда с hmi
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum IHmiCommand {
    /// Нет команды
    #[default]
    no_command,

    /// Активировать задание из hmi
    sp_hmi_en,
    /// Активировать задание из plc
    sp_plc_en,
    /// Задание из hmi
    sp_hmi(f64),
}
