use crate::executor::ComponentError;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CmpOutput: {0}")]
    CmpOutput(ComponentError),

    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("FnProcessEnd")]
    FnProcessEnd,

    #[error("TaskInputEnd")]
    TaskInputEnd,

    #[error("TaskOutputEnd")]
    TaskOutputEnd,

    #[error("AlgTaskUnexpectedEnd: {0}")]
    AlgTaskUnexpectedEnd(String),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
