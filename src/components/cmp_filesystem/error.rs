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

    #[error("WriteFileError: {0}")]
    WriteFileError(std::io::Error),

    #[error("ReadFileError: {0}")]
    ReadFileError(std::io::Error),

    #[error("ReadDirEntryError: {0}")]
    ReadDirEntryError(std::io::Error),

    #[error("CreateDirError: {0}")]
    CreateDirError(std::io::Error),

    #[error("ReadDirError: {0}")]
    ReadDirError(std::io::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
