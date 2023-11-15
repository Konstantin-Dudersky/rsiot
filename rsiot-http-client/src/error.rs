use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum Error {
    /// Ошибка конфигурации пользователя
    ConfigurationError(String),
    ReqwestError(ReqwestError),
}

impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self {
        Self::ReqwestError(value)
    }
}
