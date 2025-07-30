#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    ParseError(#[from] url::ParseError),

    #[error("cmp_timescaledb | TaskInputEnd")]
    TaskInputEnd,

    #[error("cmp_timescaledb | TaskSendToDatabase")]
    TaskSendToDatabase,

    #[error("cmp_timescaledb | TokioJoin")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("cmp_timescaledb | TokioMpsc")]
    TokioMpsc,

    #[error(transparent)]
    Format(#[from] time::error::Format),
}
