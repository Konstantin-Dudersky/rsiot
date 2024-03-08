#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    /// Ошибка конфигурации пользователя
    Configuration(String),

    Reqwest(#[from] reqwest::Error),

    SendChannel(#[from] tokio::sync::mpsc::error::SendError<TMessage>),

    TokioJoin(#[from] tokio::task::JoinError),

    ResponseCallback(#[from] anyhow::Error),

    ComponentCore(#[from] crate::executor::ComponentError),
}
