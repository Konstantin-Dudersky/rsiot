#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    #[error("Error when establishing connection: {0}")]
    Connect(gloo::utils::errors::JsError),

    #[error("JoinError: {source}")]
    TaskJoin {
        #[from]
        source: tokio::task::JoinError,
    },

    #[error("Error sending message to output channel: {source}")]
    OutputSend {
        #[from]
        source: tokio::sync::mpsc::error::SendError<TMessage>,
    },

    #[error("fn_output error: {0}")]
    FnOutput(anyhow::Error),
}
