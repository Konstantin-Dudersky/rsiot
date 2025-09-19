use super::COMPONENT_NAME;

/// Ошибки cmp_surrealdb
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SurrealDB error: {0}")]
    SurrealDB(#[from] surrealdb::Error),

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error(transparent)]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}
