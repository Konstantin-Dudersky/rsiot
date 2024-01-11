#[derive(Debug, thiserror::Error)]
pub enum Error<TMsg> {
    #[error("{source}")]
    Tungstenite {
        #[from]
        source: tokio_tungstenite::tungstenite::Error,
    },

    #[error("Error bind to port: {0}")]
    BindToPort(std::io::Error),

    #[error("{source}")]
    TokioTaskJoin {
        #[from]
        source: tokio::task::JoinError,
    },

    #[error("{source}")]
    TokioSyncMpscSend {
        #[from]
        source: tokio::sync::mpsc::error::SendError<TMsg>,
    },

    #[error("{0}")]
    FnInput(anyhow::Error),

    #[error("{0}")]
    FnOutput(anyhow::Error),
}
