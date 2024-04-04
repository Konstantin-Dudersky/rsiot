/// Ошибки cmp_http_client
#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    /// Ошибка конфигурации пользователя
    Configuration(String),

    /// Reqwest
    Reqwest(#[from] reqwest::Error),

    /// SendChannel
    SendChannel(#[from] tokio::sync::mpsc::error::SendError<TMessage>),

    /// TokioJoin
    TokioJoin(#[from] tokio::task::JoinError),

    /// ResponseCallback
    ResponseCallback(#[from] anyhow::Error),

    /// ComponentCore
    ComponentCore(#[from] crate::executor::ComponentError),
}
