use crate::{components::shared_tasks, executor::ComponentError};

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TaskInputRequest")]
    TaskInputRequest,

    #[error("TaskPeriodicRequest")]
    TaskPeriodicRequest,

    #[error("TaskProcessResponse")]
    TaskProcessResponse,

    #[error("TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("TokioJoin")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    TaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error("End execution of HTTP client: {0}")]
    TaskEndHttpClient(String),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
