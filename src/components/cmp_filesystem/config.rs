use crate::message::{Message, MsgDataBound};

/// Конфигурация cmp_filesystem
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Настройки сохранения сообщений в файловой системе
    ///
    ///
    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| vec![]
    /// ```
    pub fn_input: Vec<ConfigSave<TMsg>>,

    /// Настройки загрузки сообщений из файловой системы
    ///
    /// # Пример
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub fn_output: Vec<ConfigLoad<TMsg>>,
}

/// Настройка сохранения одного сообщения в файловой системе
#[derive(Clone)]
pub struct ConfigSave<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Название файла, в который произодится сохранение
    pub filename: String,

    /// Функция сохранения
    pub fn_save: fn(Message<TMsg>) -> Option<String>,
}

/// Настройка загрузки одного сообщения из файловой системы
#[derive(Clone)]
pub struct ConfigLoad<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Название файла, который считывается для восстановления
    pub filename: String,

    /// Функция восстановления
    pub fn_restore: fn(&str) -> Option<Message<TMsg>>,
}
