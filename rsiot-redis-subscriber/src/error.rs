use redis::RedisError;
use tokio::{sync::mpsc::error::SendError, task::JoinError};

use rsiot_messages_core::Error as MessagesError;

#[derive(Debug)]
pub enum Error {
    /// Ошибка десериализации
    DeserializeError(MessagesError),
    /// Ошибка подключения к redis
    RedisConnectionError(String),
    /// Ошибка отправки соообщения в канал mpsc
    SendChannelError(String),
    /// Ошибка получения собщения из асинхронной подписки PubSub
    GetMessageError,
    JoinError(JoinError),
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

impl<T> From<SendError<T>> for Error {
    fn from(value: SendError<T>) -> Self {
        Self::SendChannelError(value.to_string())
    }
}

impl From<JoinError> for Error {
    fn from(value: JoinError) -> Self {
        Self::JoinError(value)
    }
}
