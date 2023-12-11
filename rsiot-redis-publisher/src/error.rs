use redis::RedisError;

use rsiot_messages_core::Error as MessagesError;

#[derive(Debug)]
pub enum Error {
    /// Ошибка подключения к redis
    RedisConnectionError(String),
    /// Ошибка сериализации
    Serialization(MessagesError),
}
impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        Error::RedisConnectionError(value.to_string())
    }
}

impl From<MessagesError> for Error {
    fn from(value: MessagesError) -> Self {
        Self::Serialization(value)
    }
}
