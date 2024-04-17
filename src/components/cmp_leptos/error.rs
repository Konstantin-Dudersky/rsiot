/// Ошибки компонента cmp_leptos
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    TokioMpscSend(String),

    #[error("Tokio task join error: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    CmpOutput(crate::executor::ComponentError),

    #[error("Storage: {0}")]
    Storage(#[from] gloo::storage::errors::StorageError),
}

impl From<Error> for crate::executor::ComponentError {
    fn from(value: Error) -> Self {
        crate::executor::ComponentError::Execution(value.to_string())
    }
}
