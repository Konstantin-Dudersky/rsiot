use std::{marker::PhantomData, time::Duration};

use crate::message::MsgDataBound;

use super::{Error, QueryStat};

// ANCHOR: Config
/// Конфигурация компонента cmp_timescaledb
#[derive(Clone, Debug)]
pub struct Config<TMsg, TFnInput>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(&TMsg) -> Result<Option<Vec<String>>, Error> + Send + Sync,
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

    /// Название таблицы для сохранения данных
    pub table_name: &'static str,

    /// Отправить в базу данных, если в кеше присутствует больше указанного количества строк
    pub save_by_row_count: usize,

    /// Отправить в базу даннных, если с последнего сохранения прошло более указанного времени
    ///
    /// Предпочтительно, чтобы данные сохранялись в БД по полю `save_by_row_count`
    pub save_by_period: Duration,

    /// Функция преобразования сообщений в строки для Timescaledb
    pub fn_input: TFnInput,

    /// Удалить таблицу перед записью
    pub delete_before_write: bool,

    /// Функция для вывода статистики выполнения запросов
    pub fn_query_stat: fn(QueryStat),
}
// ANCHOR: Config
