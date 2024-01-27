#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SurrealDB error: {0}")]
    SurrealDB(#[from] surrealdb::Error),

    #[error(transparent)]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}
