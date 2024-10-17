use serde::{Deserialize, Serialize};

use super::super::select_mode;

/// Входная структура
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

    /// false - блокировка работы
    pub intlock: bool,

    /// Команда с hmi
    pub hmi_command: IHmiCommand,

    /// Команда на запуск в режиме auto
    pub auto_start: bool,
    /// Команда на останов в режиме stop
    pub auto_stop: bool,
}

/// Команда с hmi
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum IHmiCommand {
    /// Нет команды - по-умолчанию
    #[default]
    no_command,

    /// Запуск
    man_start,
    /// Стоп
    man_stop,

    /// Переключение в режим man из hmi
    mode_man,
    /// Переключение в режим auto из hmi
    mode_auto,
    /// Переключение в режим local из hmi
    mode_local,
    /// Переключение в режим oos из hmi
    mode_oos,
}

impl From<IHmiCommand> for select_mode::IHmiCommand {
    fn from(value: IHmiCommand) -> Self {
        match value {
            IHmiCommand::no_command => select_mode::IHmiCommand::no_command,

            IHmiCommand::mode_man => select_mode::IHmiCommand::mode_man,
            IHmiCommand::mode_auto => select_mode::IHmiCommand::mode_auto,
            IHmiCommand::mode_local => select_mode::IHmiCommand::mode_local,
            IHmiCommand::mode_oos => select_mode::IHmiCommand::mode_oos,

            _ => select_mode::IHmiCommand::no_command,
        }
    }
}
