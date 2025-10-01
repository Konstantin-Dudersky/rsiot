use crate::message::MsgDataBound;

/// Конфигурация компонента cmp_os_process
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Вектор команд
    pub commands: Vec<Command<TMsg>>,
}

/// Конфигурация отдельной команды
#[derive(Clone)]
pub struct Command<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Функция преобразования входящих сообщений в вектор команд
    pub fn_input: fn(&TMsg) -> Option<Vec<String>>,

    /// Функция преобразования вывода команд в вектор сообщений
    pub fn_output: fn(&[ExecResult]) -> Option<Vec<TMsg>>,
}

pub struct ExecResult {
    pub status: String,
    pub stdout: String,
    pub stderr: String,
}
