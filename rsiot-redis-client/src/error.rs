use tokio::sync::mpsc::error::SendError;

#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    /// Ошибка получения собщения из асинхронной подписки PubSub
    #[error("Error redis subscription")]
    GetMessage,

    #[error("Error in async task: {source}")]
    Join {
        #[from]
        source: tokio::task::JoinError,
    },

    /// Ошибка десериализации
    #[error("Error in message serialization / deserialization: {source:?}")]
    MessageError {
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
        source: SendError<TMessage>,
    },
    // SendChannel(String),
}
