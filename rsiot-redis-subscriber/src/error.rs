use redis::RedisError;
use tokio::{sync::mpsc::error::SendError, task::JoinError};

use rsiot_messages_core::Error as MessagesError;

#[derive(Debug)]
pub enum Error {
    /// Ошибка десериализации
    Deserialize(MessagesError),
    /// Ошибка подключения к redis
    RedisConnection(String),
    /// Ошибка отправки соообщения в канал mpsc
    SendChannel(String),
    /// Ошибка получения собщения из асинхронной подписки PubSub
    GetMessage,
    Join(JoinError),
}

impl From<MessagesError> for Error {
    fn from(value: MessagesError) -> Self {
        Self::Deserialize(value)
    }
}

impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        Error::RedisConnection(value.to_string())
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(value: SendError<T>) -> Self {
        Self::SendChannel(value.to_string())
    }
}

impl From<JoinError> for Error {
    fn from(value: JoinError) -> Self {
        Self::Join(value)
    }
}
