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

    /// Команда с hmi
    pub hmi_command: IHmiCommand,
}

/// Команда с hmi
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
}
