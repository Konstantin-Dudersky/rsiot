/// Ошибки cmp_surrealdb
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// SurrealDB
    #[error("SurrealDB error: {0}")]
    SurrealDB(#[from] surrealdb::Error),

    /// TokioTaskJoin
    #[error(transparent)]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}
