use crate::{components::shared_tasks, executor::ComponentError};

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

    #[error(transparent)]
    TaskMpscToMsgbus(shared_tasks::mpsc_to_msgbus::Error),

    #[error(transparent)]
    TaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error(transparent)]
    TaskMpscToBroadcast(shared_tasks::mpsc_to_broadcast::Error),

    #[error("AlgTaskUnexpectedEnd: {0}")]
    AlgTaskUnexpectedEnd(String),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
