#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    #[error("{0}")]
    Connection(#[from] std::io::Error),

    #[error("Modbus request error. Request: {request:?}. Error: {error}")]
    Request {
        request: crate::config::Request,
        error: String,
    },

    #[error("{0}")]
    TokioSyncMpsc(#[from] tokio::sync::mpsc::error::SendError<TMessage>),

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}
