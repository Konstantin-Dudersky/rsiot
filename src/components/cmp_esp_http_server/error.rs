use esp_idf_svc::io::EspIOError;

use crate::executor::ComponentError;

/// Ошибки cmp_esp_http_server
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TaskEndUpdateGetEndpoints")]
    TaskEndUpdateGetEndpoints,

    #[error("Unknown path: {0}")]
    UnknownPath(String),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    RequestIntoResponse(EspIOError),

    #[error(transparent)]
    ResponseWriteAll(EspIOError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
