use serde::{Deserialize, Serialize};

use crate::components::cmp_plc::plc::library::drives::{select_mode, select_sp};

/// Входная структура
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    /// Источник выбора режима:
    /// - false => из hmi,
    /// - true => из plc
    pub mode_source: bool,
    /// Переключение в режим auto из контроллера
    pub mode_auto: bool,
    /// Переключение в режим manual из контроллера
    pub mode_man: bool,
    /// Переключение в режим local из контроллера
    pub mode_local: bool,
    /// Переключение в режим oos из контроллера
    pub mode_oos: bool,

    /// Выбор источника команд выбора (`sp_hmi_en`, `sp_plc_en`):
    /// false => из hmi
    /// true => из plc
    pub mv_en_source: bool,
    /// Активировать задание из hmi
    pub mv_hmi_en: bool,
    /// Активировать задание из plc
    pub mv_plc_en: bool,
    /// Задание из plc
    pub mv_plc: f64,

    /// Фактическое открытие
    pub rbk: f64,

    /// Команда с hmi
    pub hmi_command: IHmiCommand,
}

/// Команда с hmi
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum IHmiCommand {
    /// Нет команды - по-умолчанию
    #[default]
    no_command,

    /// Переключение в режим man из hmi
    mode_man,
    /// Переключение в режим auto из hmi
    mode_auto,
    /// Переключение в режим local из hmi
    mode_local,
    /// Переключение в режим oos из hmi
    mode_oos,

    /// Активировать задание из hmi
    mv_hmi_en,
    /// Активировать задание из plc
    mv_plc_en,
    /// Задание из hmi
    mv_hmi(f64),
}

impl From<IHmiCommand> for select_sp::IHmiCommand {
    fn from(value: IHmiCommand) -> Self {
        match value {
            IHmiCommand::mv_hmi_en => select_sp::IHmiCommand::sp_hmi_en,
            IHmiCommand::mv_plc_en => select_sp::IHmiCommand::sp_plc_en,
            IHmiCommand::mv_hmi(mv) => select_sp::IHmiCommand::sp_hmi(mv),
            _ => select_sp::IHmiCommand::no_command,
        }
    }
}

impl From<IHmiCommand> for select_mode::IHmiCommand {
    fn from(value: IHmiCommand) -> Self {
        match value {
            IHmiCommand::mode_man => select_mode::IHmiCommand::mode_man,
            IHmiCommand::mode_auto => select_mode::IHmiCommand::mode_auto,
            IHmiCommand::mode_local => select_mode::IHmiCommand::mode_local,
            IHmiCommand::mode_oos => select_mode::IHmiCommand::mode_oos,
            _ => select_mode::IHmiCommand::no_command,
        }
    }
}
