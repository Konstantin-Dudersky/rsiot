#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("{source}")]
    GlooNet {
        #[from]
        source: gloo::net::Error,
    },

    #[error("Error when process on_success callback: {0}")]
    OnSuccess(anyhow::Error),

    #[error("{source}")]
    TokioSyncMpscSend {
        #[from]
        source: tokio::sync::mpsc::error::SendError<TMessage>,
    },

    #[error("{source}")]
    TokioTaskJoin {
        #[from]
        source: tokio::task::JoinError,
    },
}
