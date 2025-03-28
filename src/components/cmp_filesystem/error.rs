use crate::executor::ComponentError;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CmpOutput: {0}")]
    CmpOutput(ComponentError),

    #[error("FnOutput: {0}")]
    FnInput(anyhow::Error),

    #[error("FnOutput: {0}")]
    FnOutput(anyhow::Error),

    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("WriteFileError: {0}; filename: {1}")]
    WriteFileError(std::io::Error, String),

    #[error("ReadFileError: {0}")]
    ReadFileError(std::io::Error),

    #[error("ReadDirEntryError: {0}")]
    ReadDirEntryError(std::io::Error),

    #[error("TokioMpscSend")]
    TokioMpscSend,

    #[error(transparent)]
    Serde(#[from] crate::serde_utils::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
