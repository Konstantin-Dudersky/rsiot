use url::Url;

use crate::row::Row;

#[derive(Clone, Debug)]
pub struct Config<TMessage> {
    /// Функция преобразования сообщений в модель строк БД
    pub fn_process: fn(TMessage) -> Option<Row>,
    /// Строка подключения к БД
    pub connection_string: Url,
}
