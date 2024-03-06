use rsiot_component_core::ComponentError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    TokioMpscSend(String),

    #[error("Tokio task join error: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    CmpOutput(rsiot_component_core::ComponentError),

    #[error("Storage: {0}")]
    Storage(#[from] gloo::storage::errors::StorageError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
