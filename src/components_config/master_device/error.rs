use crate::components::shared_tasks;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("master_device | RequestKindUnknown: {0}")]
    RequestKindUnknown(u8),

    #[error("master_device | TaskFilterIdenticalData: {0}")]
    TaskFilterIdenticalData(shared_tasks::filter_identical_data::Error),

    #[error("master_device | TokioTaskJoin: \nsource: {source}")]
    TokioTaskJoin {
        #[from]
        source: tokio::task::JoinError,
    },

    #[error("master_device | TokioSyncMpsc")]
    TokioSyncMpscSend,

    /// Закончилось выполнение
    #[error("master_device | EndExecution")]
    EndExecution,
}
