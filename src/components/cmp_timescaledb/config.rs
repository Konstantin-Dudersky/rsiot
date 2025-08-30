use std::time::Duration;

use crate::message::MsgDataBound;

use super::Row;

pub type FnInput<TMsg> = fn(&TMsg) -> Option<Vec<Row>>;

/// Конфигурация компонента cmp_timescaledb
#[derive(Clone, Debug)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Строка подключения к БД
    ///
    /// Примеры:
    ///
    /// - ```String::from("postgres://user:password@localhost:5432/db_name")```
    pub connection_string: String,

    /// Максимальное количество подключений к БД
    pub max_connections: u32,

    /// Название таблицы для сохранения данных
    pub table_name: &'static str,

    /// Периодичность отправки данных для сохранения в базе данных
    pub send_period: Duration,

    /// Функция преобразования сообщений в строки для Timescaledb
    pub fn_input: FnInput<TMsg>,

    /// Удалить таблицу перед записью
    pub delete_before_write: bool,
}
