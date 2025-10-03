use crate::message::MsgDataBound;

/// Конфигурация компонента cmp_os_process
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Вектор команд
    pub commands: Vec<ConfigCommand<TMsg>>,
}

/// Конфигурация отдельной команды
#[derive(Clone)]
pub struct ConfigCommand<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Функция преобразования входящих сообщений в вектор команд
    pub fn_input: fn(&TMsg) -> Option<Vec<String>>,

    /// Функция преобразования вывода команд в вектор сообщений
    pub fn_output: fn(&[ExecResult]) -> Option<Vec<TMsg>>,
}

/// Результат выполнения команды
pub struct ExecResult {
    /// Статус выполнения команды
    pub status: String,

    /// Вывод команды
    pub stdout: String,

    /// Ошибки команды
    pub stderr: String,
}
