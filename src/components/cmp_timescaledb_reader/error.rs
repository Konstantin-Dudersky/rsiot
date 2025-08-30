use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | PConnectionError: {0}")]
    PgConnectionError(sqlx::Error),

    #[error("{COMPONENT_NAME} | TimeFormat: {0}")]
    TimeFormat(#[from] time::error::Format),

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend: {0}")]
    TokioSyncAcquire(#[from] tokio::sync::AcquireError),

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | TryNext: {0}")]
    TryNext(String),

    #[error("{COMPONENT_NAME} | UrlParseError: {0}")]
    UrlParseError(#[from] url::ParseError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
