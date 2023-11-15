#[derive(Debug)]
pub enum Error {
    /// Ошибка конфигурации пользователя
    ConfigurationError(String),
}
