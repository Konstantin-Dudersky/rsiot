use std::time::Duration;

use time::OffsetDateTime;

use crate::message::{MsgDataBound, ValueTime};

/// Конфигурация cmp_timescaledb_reader
#[derive(Clone)]
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

    /// Начало диапазона времени
    pub time_begin: OffsetDateTime,

    /// Конец диапазона времени
    pub time_end: OffsetDateTime,

    /// Настройки параметров
    pub items: Vec<ConfigItem<TMsg>>,

    /// Задержка между отправкой сообщений в шину. Слишком много сообщений может привести к
    /// переполнению шины
    pub delay_between_msgs: Duration,

    /// Задержка перед окончанием работы
    pub shutdown_delay: Duration,
}

/// Конфигурация отдельного параметра
#[derive(Clone)]
pub struct ConfigItem<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Сущность
    pub entity: &'static str,

    /// Атрибут
    pub attr: &'static str,

    /// Функция создания исходящих сообщений
    pub fn_output: fn(ValueTime) -> TMsg,
}
