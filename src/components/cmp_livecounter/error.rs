use crate::{components::shared_tasks, executor::ComponentError};

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    TaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error("TaskInput")]
    TaskInput,

    #[error("TaskCheck")]
    TaskCheck,
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
