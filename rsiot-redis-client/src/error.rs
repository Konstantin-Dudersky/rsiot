#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    #[error("Error in async task: {source}")]
    Join {
        #[from]
        source: tokio::task::JoinError,
    },

    /// Ошибка десериализации
    #[error("Error in message serialization / deserialization: {source:?}")]
    Message {
        #[from]
        source: rsiot_messages_core::Error,
    },

    /// Ошибка подключения к redis
    #[error("Redis connection error: {source}")]
    RedisConnection {
        #[from]
        source: redis::RedisError,
    },

    /// Ошибка отправки соообщения в канал mpsc
    #[error("Error sending message to channel: {source}")]
    SendChannel {
        #[from]
        source: tokio::sync::mpsc::error::SendError<TMessage>,
    },
}
