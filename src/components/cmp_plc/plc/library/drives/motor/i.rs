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

    /// Команда на запуск в режиме auto
    pub auto_start: bool,
    /// Команда на останов в режиме stop
    pub auto_stop: bool,
}

/// Команда с hmi
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    /// Переключение в режим local из hmi
    LocalMode,
}
