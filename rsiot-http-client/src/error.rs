#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    /// Ошибка конфигурации пользователя
    Configuration(String),

    Reqwest {
        #[from]
        source: reqwest::Error,
    },

    SendChannel {
        #[from]
        source: tokio::sync::mpsc::error::SendError<TMessage>,
    },

    TokioJoin {
        #[from]
        source: tokio::task::JoinError,
    },

    ResponseCallback {
        #[from]
        source: anyhow::Error,
    },
}
