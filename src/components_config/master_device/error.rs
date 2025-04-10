use crate::components::shared_tasks;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    TaskFilterIdenticalData(shared_tasks::filter_identical_data::Error),

    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("TokioSyncMpsc")]
    TokioSyncMpsc,
}
