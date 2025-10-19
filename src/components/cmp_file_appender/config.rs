use std::marker::PhantomData;

use crate::message::MsgDataBound;

/// Конфигурация компонента cmp_file_appender
#[derive(Clone)]
pub struct Config<TMsg, TFnInput>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(TMsg) -> ConfigAction + Send + Sync,
{
    /// PhantomData для сообщений
    pub _msg_phantom: PhantomData<TMsg>,

    /// Функция преобразования сообщений в строки для сохранения в файл
    pub fn_input: TFnInput,
}

/// Действие на основе входящего сообщения
#[derive(Clone)]
pub enum ConfigAction {
    /// Не предпринимать никаких действий
    NoAction,

    /// Добавить строку в файл
    AppendLine {
        /// Название файла
        filename: String,

        /// Строка для добавления в файл
        line: String,
    },

    /// Завершить обработку
    EndProcessing,
}
