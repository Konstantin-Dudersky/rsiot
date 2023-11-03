use redis::RedisError;

use rsiot_messages_core::Errors as MessagesError;

#[derive(Debug)]
pub enum Error {
    RedisConnectionError(String),
    /// Ошибка десериализации
    DeserializeError(MessagesError),
    /// Ошибка отправки соообщения в канал mpsc
    SendThreadChannleError(String),
    /// Ошибка получения собщения из асинхронной подписки PubSub
    GetMessageError,
}

impl From<MessagesError> for Error {
    fn from(value: MessagesError) -> Self {
        Self::DeserializeError(value)
    }
}

impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        Error::RedisConnectionError(value.to_string())
    }
}
