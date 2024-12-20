use std::io::Error as StdIoError;

use axum::{http::StatusCode, response::IntoResponse};

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Ошибка Axum
    #[error(transparent)]
    AxumServe(StdIoError),

    /// Ошибка привязки к порту
    #[error(transparent)]
    BindPort(StdIoError),

    #[error("{0}")]
    UnknownMessageKey(String),

    #[error("{0}")]
    NotConfigured(String),

    #[error(transparent)]
    Message(#[from] crate::message::Error),

    #[error(transparent)]
    FnInput(anyhow::Error),

    #[error(transparent)]
    FnOutput(anyhow::Error),

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
}

/// Преобразование ошибки в понятный пользователю ответ
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            Error::AxumServe(err) => format!("{:?}", err),
            Error::BindPort(err) => format!("{:?}", err),
            Error::Message(err) => format!("{:?}", err),
            Error::UnknownMessageKey(key) => {
                format!("Unknown message key: {}", key)
            }
            Error::FnInput(err) => format!("{}", err),
            Error::FnOutput(err) => format!("{}", err),
            Error::CmpOutput(err) => format!("{}", err),
            Error::NotConfigured(err) => format!("Not configured: {}", err),
            Error::TaskEndAxumServe => format!("{self}"),
            Error::TaskEndCmpPlcInput => format!("{self}"),
            Error::TaskEndCmpPlcOutput => format!("{self}"),
            Error::TaskEndCmpPlcStatic => format!("{self}"),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
