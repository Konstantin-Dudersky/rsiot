#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Connection(#[from] std::io::Error),

    #[error("Modbus request error. Request: {request:?}. Error: {error}")]
    Request {
        request: crate::config::Request,
        error: String,
    },

    #[error(transparent)]
    CmpOutput(rsiot_component_core::ComponentError),

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}
