use serde::{Deserialize, Serialize};

/// Входная структура
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    /// Переключение в режим auto из контроллера
    pub auto_mode_plc: bool,
    /// Переключение в режим man из контроллера
    pub man_mode_plc: bool,

    /// Команда с hmi
    pub hmi_command: IHmiCommand,

    /// Источник выбора режима: 0 = из plc, 1 = из hmi
    pub mode_plc_hmi: bool,

    /// Команда на запуск в режиме auto
    pub auto_start: bool,
    /// Команда на останов в режиме stop
    pub auto_stop: bool,
}

/// Команда с hmi
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum IHmiCommand {
    /// Нет команды - по-умолчанию
    #[default]
    NoCommand,
    /// Запуск
    ManStart,
    /// Стоп
    ManStop,
    /// Переключение в режим man из hmi
    ManMode,
    /// Переключение в режим auto из hmi
    AutoMode,
}
