use crate::{components::shared_tasks, executor::ComponentError};

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
