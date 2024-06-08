use serde::{Deserialize, Serialize};

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

    /// Команда с hmi
    pub hmi_command: IHmiCommand,
}

/// Команда с hmi
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
}
