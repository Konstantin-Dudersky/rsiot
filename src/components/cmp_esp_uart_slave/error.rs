use crate::{components::shared_tasks, executor::ComponentError, serde_utils::postcard_serde};

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

    #[error("FnUartComm: {0}")]
    FnUartComm(anyhow::Error),

    #[error("TaskOutput: {0}")]
    TaskOutput(String),

    #[error("TaskFilterIdenticalData: {0}")]
    TaskFilterIdenticalData(shared_tasks::filter_identical_data::Error),

    #[error("TaskMpscToMsgbus: {0}")]
    TaskMpscToMsgbus(shared_tasks::mpsc_to_msgbus::Error),

    #[error(transparent)]
    Postcard(#[from] postcard_serde::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
