use crate::{components::shared_tasks, executor::ComponentError};

/// Errors of cmp_slint
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cmp_slint | SlintEventLoopError: {0}")]
    SlintEventLoopError(#[from] slint::EventLoopError),

    #[error("cmp_slint | TaskFilterSendPeriodically: {0}")]
    TaskFilterSendPeriodically(shared_tasks::filter_send_periodically::Error),

    #[error("cmp_slint | TaskInput")]
    TaskInput,

    #[error("cmp_slint | TaskMpscToMsgBus: {0}")]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error("cmp_slint | TaskMsgBusToMpsc: {0}")]
    TaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error("cmp_slint | TaskOutput")]
    TaskOutput,

    #[error("cmp_slint | TokioJoin: {0}")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("cmp_slint | TokioSyncMpsc")]
    TokioSyncMpsc,
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
