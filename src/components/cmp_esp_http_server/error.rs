use esp_idf_svc::{io::EspIOError, sys::EspError};

use crate::{executor::ComponentError, serde_utils};

/// Ошибки cmp_esp_http_server
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TaskEndUpdateGetEndpoints")]
    TaskEndUpdateGetEndpoints,

    #[error("Unknown path: {0}")]
    UnknownPath(String),

    #[error(transparent)]
    Component(#[from] ComponentError),

    #[error(transparent)]
    Serde(#[from] serde_utils::Error),

    #[error(transparent)]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("RegisterHandler")]
    RegisterHandler(EspError),

    #[error("RequestContentLen")]
    RequestContentLen,

    #[error(transparent)]
    RequestIntoResponse(EspIOError),

    #[error("RequestReadBody")]
    RequestReadBody(String),

    #[error(transparent)]
    ResponseWriteAll(EspIOError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
