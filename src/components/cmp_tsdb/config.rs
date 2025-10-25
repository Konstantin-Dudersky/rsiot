use std::{marker::PhantomData, time::Duration};

use crate::message::MsgDataBound;

use super::{Error, Row};

// ANCHOR: Config
/// Конфигурация компонента cmp_timescaledb
#[derive(Clone, Debug)]
pub struct Config<TMsg, TFnInput>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(&TMsg) -> Result<Option<Vec<Row>>, Error> + Send + Sync,
{
    /// PhantomData для сообщений
    pub _msg_phantom: PhantomData<TMsg>,

    /// Строка подключения к БД
    ///
    /// Примеры:
    ///
    /// - ```String::from("postgres://user:password@localhost:5432/db_name")```
    pub connection_string: String,

    /// Максимальное количество подключений к БД
    ///
    /// Можно выставить значение "10"
    pub max_connections: u32,

    /// Максимальный размер кэша для хранения данных
    ///
    /// Можно выставить значение "10_000"
    pub max_cache_size: usize,

    /// Название таблицы для сохранения данных
    pub table_name: &'static str,

    /// Периодичность отправки данных для сохранения в базе данных
    pub send_period: Duration,

    /// Функция преобразования сообщений в строки для Timescaledb
    pub fn_input: TFnInput,

    /// Удалить таблицу перед записью
    pub delete_before_write: bool,
}
// ANCHOR: Config
