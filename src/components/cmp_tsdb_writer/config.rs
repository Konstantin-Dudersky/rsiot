use std::time::Duration;

use crate::message::MsgDataBound;

use super::{Error, QueryStat};

// ANCHOR: Config
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
    pub connection_string: ConfigConnectionString,

    /// Максимальное количество подключений к БД
    ///
    /// Можно выставить значение "10"
    pub max_connections: u32,

    /// Название таблицы для сохранения данных
    pub tables: Vec<ConfigTable<TMsg>>,

    /// Отправить в базу данных, если в кеше присутствует больше указанного количества строк
    pub save_by_row_count: usize,

    /// Отправить в базу даннных, если с последнего сохранения прошло более указанного времени
    ///
    /// Предпочтительно, чтобы данные сохранялись в БД по полю `save_by_row_count`
    pub save_by_period: Duration,

    /// Функция для вывода статистики выполнения запросов
    pub fn_query_stat: fn(QueryStat),
}
// ANCHOR: Config

/// Строка подключения к БД
#[derive(Clone, Debug)]
pub struct ConfigConnectionString {
    /// Пользователь
    ///
    /// `postgres`
    pub user: String,

    /// Пароль
    ///
    /// `postgres`
    pub password: String,

    /// Хост
    ///
    /// `localhost`
    pub database_host: String,

    /// Порт
    ///
    /// `5432`
    pub port: u16,

    /// Проект
    pub prj: String,

    /// Хост
    pub hst: String,

    /// Сервис
    pub svc: String,
}
impl ConfigConnectionString {
    /// Возвращает строку подключения к БД
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}__{}__{}",
            self.user, self.password, self.database_host, self.port, self.prj, self.hst, self.svc
        )
    }
}

/// Конфигурация таблицы для сохранения данных в БД
#[derive(Clone, Debug)]
pub struct ConfigTable<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Компонент
    pub cmp: String,

    /// Тег
    pub tag: String,

    /// Разделение на части по времени
    pub chunk_interval: Duration,

    /// Сжимать данные через указанный интервал после записи
    pub compress_interval: Duration,

    /// Удалять данные старше указанного интервала. Если не указано, то данные не удаляются
    pub retention_interval: Option<Duration>,

    /// Поля таблицы
    pub fields: Vec<ConfigTableField>,

    /// Функция преобразования сообщений в строки для Timescaledb
    pub fn_input: fn(&TMsg) -> Result<Option<String>, Error>,

    /// Удалить таблицу перед записью
    pub delete_before_write: bool,
}
impl<TMsg> ConfigTable<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Название таблицы в БД
    pub fn table_name(&self) -> String {
        format!("{}__{}", self.cmp, self.tag)
    }
}

/// Конфигурация поля таблицы
#[derive(Clone, Debug)]
pub struct ConfigTableField {
    /// Название поля таблицы
    pub field_name: String,

    /// Тип поля таблицы
    pub data_type: ConfigTableFieldType,
}

/// Тип поля таблицы
#[derive(Clone, Debug)]
pub enum ConfigTableFieldType {
    /// Произвольный текст
    CharacterText,

    /// 4 байта
    NumericInteger,

    /// 8 байт
    NumericDoublePrecision,

    /// 1 байт
    Boolean,
}
impl ConfigTableFieldType {
    /// Преобразование типа поля в тип PostgreSQL
    pub fn into_pg_type(&self) -> &'static str {
        match self {
            Self::CharacterText => "TEXT",
            Self::NumericInteger => "INTEGER",
            Self::NumericDoublePrecision => "DOUBLE PRECISION",
            Self::Boolean => "BOOLEAN",
        }
    }
}

pub(crate) struct ConfigTableForSetup {
    pub table_name: String,
    pub delete_before_write: bool,
    pub chunk_interval: Duration,
    pub compress_interval: Duration,
    pub retention_interval: Option<Duration>,
    pub values: Vec<ConfigTableField>,
}

impl<TMsg> From<&ConfigTable<TMsg>> for ConfigTableForSetup
where
    TMsg: MsgDataBound,
{
    fn from(config_table: &ConfigTable<TMsg>) -> Self {
        ConfigTableForSetup {
            table_name: config_table.table_name(),
            delete_before_write: config_table.delete_before_write,
            chunk_interval: config_table.chunk_interval,
            compress_interval: config_table.compress_interval,
            retention_interval: config_table.retention_interval,
            values: config_table.fields.clone(),
        }
    }
}
