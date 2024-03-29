use crate::executor::ComponentError;

/// Ошибки cmp_plc
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// CmpOutput
    #[error(transparent)]
    CmpOutput(ComponentError),

    /// TokioTaskJoin
    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
