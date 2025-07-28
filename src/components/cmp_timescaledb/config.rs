use std::time::Duration;

use crate::message::MsgDataBound;

use super::Row;

pub type FnInput<TMsg> = fn(&TMsg) -> Option<Vec<Row>>;

/// Конфигурация Timescaledb
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

    /// Периодичность отправки данных для сохранения в базе данных
    pub send_period: Duration,

    /// Функция преобразования сообщений в строки для Timescaledb
    pub fn_input: FnInput<TMsg>,
}
