use crate::message::{Message, MsgDataBound};

/// Функция преобразования сообщений в текстовые файлы.
///
/// Возращает кортеж из двух значений:
/// - название файла для сохранения
/// - содержимое файла
pub type FnInput<TMsg> = fn(Message<TMsg>) -> anyhow::Result<Option<(String, String)>>;

/// Функция преобразования текстовых файлов в сообщения
pub type FnOutput<TMsg> = fn(&str) -> anyhow::Result<Option<Message<TMsg>>>;

/// Конфигурация cmp_filesystem
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Папка, в которой хранятся файлы
    pub directory: String,

    /// Функция преобразования сообщений в текстовые файлы
    pub fn_input: FnInput<TMsg>,

    /// Функция преобразования текстовых файлов в сообщения
    pub fn_output: FnOutput<TMsg>,
}
