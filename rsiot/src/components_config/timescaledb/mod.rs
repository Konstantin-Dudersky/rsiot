use url::Url;

/// Конфигурация Timescaledb
#[derive(Clone, Debug)]
pub struct Config {
    /// Строка подключения к БД
    ///
    /// Примеры:
    ///
    /// - ```Url::parse("postgres://user:password@localhost:5432/db_name")?```
    pub connection_string: Url,
}
