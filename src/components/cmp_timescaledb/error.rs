use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Format(#[from] time::error::Format),

    #[error("{COMPONENT_NAME} | Internal buffer too large: {0}")]
    LargeInternalBuffer(usize),

    #[error(transparent)]
    ParseError(#[from] url::ParseError),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error("{COMPONENT_NAME} | Error spawning task")]
    Spawn(std::io::Error),

    #[error("{COMPONENT_NAME} | TaskInputEnd")]
    TaskInputEnd,

    #[error("{COMPONENT_NAME} | TaskSendToDatabase")]
    TaskSendToDatabase,

    #[error("{COMPONENT_NAME} | TokioJoin")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | TokioMpsc in task: {task_name}")]
    TokioMpsc { task_name: &'static str },
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
