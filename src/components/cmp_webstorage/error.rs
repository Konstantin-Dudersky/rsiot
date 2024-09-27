use crate::components::shared_tasks;

/// Ошибки компонента cmp_webstorage
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Component: {0}")]
    Component(#[from] crate::executor::ComponentError),

    #[error("FnInput: {0}")]
    FnInput(anyhow::Error),

    #[error("FnInput: {0}")]
    FnOutput(anyhow::Error),

    #[error("Storage: {0}")]
    Storage(#[from] gloo::storage::errors::StorageError),

    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("TokioSyncMpsc")]
    TokioSyncMpsc(String),

    // Ошибки в задачах ----------------------------------------------------------------------------
    #[error(transparent)]
    TaskMsgBusToMpsc(shared_tasks::msg_bus_to_mpsc::Error),

    #[error("TaskInput")]
    TaskEndInput,

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msg_bus::Error),
}
