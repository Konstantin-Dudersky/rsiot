#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    TokioSynBroadcast(String),

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}
