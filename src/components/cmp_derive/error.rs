/// Ошибки cmp_derive
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// TokioSynBroadcast
    #[error("{0}")]
    TokioSynBroadcast(String),

    /// TokioTaskJoin
    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}
