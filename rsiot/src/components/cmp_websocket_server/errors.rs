#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Error bind to port: {0}")]
    BindToPort(std::io::Error),

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{0}")]
    TokioSyncMpsc(String),

    #[error("{0}")]
    FnInput(anyhow::Error),

    #[error("Error: {err}, text from client: {data}")]
    FnOutput { err: anyhow::Error, data: String },

    #[error("Client disconnected")]
    ClientDisconnected,

    #[error(transparent)]
    CmpOutput(crate::executor::ComponentError),
}

impl<TMsg> From<tokio::sync::mpsc::error::SendError<TMsg>> for Error {
    fn from(value: tokio::sync::mpsc::error::SendError<TMsg>) -> Self {
        Self::TokioSyncMpsc(value.to_string())
    }
}
