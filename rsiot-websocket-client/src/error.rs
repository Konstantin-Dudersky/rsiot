#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    #[error("Error from tunstenite: {source}")]
    Tungstenite {
        #[from]
        source: tokio_tungstenite::tungstenite::Error,
    },

    #[error("{source}")]
    TokioTaskJoin {
        #[from]
        source: tokio::task::JoinError,
    },

    #[error("{source}")]
    TokioMpscSend {
        #[from]
        source: tokio::sync::mpsc::error::SendError<TMessage>,
    },

    #[error("fn_input error: {0}")]
    FnInput(anyhow::Error),

    #[error("fn_output error: {0}")]
    FnOutput(anyhow::Error),
}
