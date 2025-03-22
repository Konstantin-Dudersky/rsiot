use crate::{components::shared_tasks, executor::ComponentError};

/// Errors of cmp_slint
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SlintEventLoopError(#[from] slint::EventLoopError),

    #[error(transparent)]
    TaskFilterSendPeriodically(shared_tasks::filter_send_periodically::Error),

    #[error("TaskInput")]
    TaskInput,

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error(transparent)]
    TaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error("TaskOutput")]
    TaskOutput,

    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("TokioSyncMpsc")]
    TokioSyncMpsc,
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
