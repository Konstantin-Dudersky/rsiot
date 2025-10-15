use crate::message::MsgDataBound;

/// Функция преобразования сообщений в строки для сохранения в файл
pub type FnInput<TMsg> = fn(&TMsg) -> ConfigAction;

/// Конфигурация компонента cmp_file_appender
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Название файла
    pub filename: String,

    /// Функция преобразования сообщений в строки для сохранения в файл
    pub fn_input: FnInput<TMsg>,
}

/// Действие на основе входящего сообщения
#[derive(Clone)]
pub enum ConfigAction {
    /// Не предпринимать никаких действий
    NoAction,

    /// Добавить строку в файл
    AppendLine(String),

    /// Завершить обработку
    EndProcessing,
}
