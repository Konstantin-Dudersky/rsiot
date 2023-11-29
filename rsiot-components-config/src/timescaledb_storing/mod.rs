use url::Url;

#[derive(Clone, Debug)]
pub struct Config {
    /// Строка подключения к БД
    pub connection_string: Url,
}
