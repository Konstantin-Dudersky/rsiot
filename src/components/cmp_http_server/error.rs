use std::io::Error as StdIoError;

use axum::{http::StatusCode, response::IntoResponse};

use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Ошибка Axum
    #[error(transparent)]
    AxumServe(StdIoError),

    /// Ошибка привязки к порту
    #[error(transparent)]
    BindPort(StdIoError),

    #[error("Unknown path: {0}")]
    UnknownPath(String),

    #[error("Not configured: {0}")]
    NotConfigured(String),

    #[error(transparent)]
    Message(#[from] crate::message::Error),

    #[error(transparent)]
    FnInput(anyhow::Error),

    #[error(transparent)]
    FnOutput(anyhow::Error),

    #[error("HeaderParseError: {0}")]
    InvalidHeaderValue(axum::http::header::InvalidHeaderValue),

    #[error(transparent)]
    CmpOutput(#[from] crate::executor::ComponentError),

    #[error("TaskEndAxumServe")]
    TaskEndAxumServe,

    #[error("TaskEndCmpPlcInput")]
    TaskEndCmpPlcInput,

    #[error("TaskEndCmpPlcOutput")]
    TaskEndCmpPlcOutput,

    #[error("TaskEndCmpPlcStatic")]
    TaskEndCmpPlcStatic,

    #[error("UpdateGetEndpoints")]
    TaskUpdateGetEndpoints,

    #[error(transparent)]
    Serde(#[from] crate::serde_utils::Error),

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

/// Преобразование ошибки в понятный пользователю ответ
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let body = self.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
