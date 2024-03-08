#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error in async task: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("End redis subscription")]
    EndRedisSubscription,

    /// Ошибка десериализации
    #[error("Error in message serialization / deserialization: {0}")]
    Message(#[from] crate::message::Error),

    /// Ошибка подключения к redis
    #[error("Redis connection error: {0}")]
    RedisConnection(#[from] redis::RedisError),

    #[error(transparent)]
    CmpOutput(crate::executor::ComponentError),

    #[error(transparent)]
    FnInput(anyhow::Error),

    #[error("FnOutput: {0}")]
    FnOutput(anyhow::Error),
}
