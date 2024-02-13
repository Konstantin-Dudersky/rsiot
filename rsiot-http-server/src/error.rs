use std::io::Error as StdIoError;

use axum::{http::StatusCode, response::IntoResponse};
use tokio::sync::mpsc::error::SendError;

use rsiot_messages_core::Error as MessageError;

#[derive(Debug, thiserror::Error)]
pub enum Error<TMessage> {
    /// Ошибка Axum
    AxumServe(StdIoError),
    /// Ошибка привязки к порту
    BindPort(StdIoError),
    UnknownMessageKey(String),
    Message(MessageError),
    ChannelSend(SendError<TMessage>),

    FnInput(anyhow::Error),
    FnOutput(anyhow::Error),
}

impl<TMessage> From<MessageError> for Error<TMessage> {
    fn from(value: MessageError) -> Self {
        Self::Message(value)
    }
}

impl<TMessage> From<SendError<TMessage>> for Error<TMessage> {
    fn from(value: SendError<TMessage>) -> Self {
        Self::ChannelSend(value)
    }
}

/// Преобразование ошибки в понятный пользователю ответ
impl<TMessage> IntoResponse for Error<TMessage> {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            Error::AxumServe(err) => format!("{:?}", err),
            Error::BindPort(err) => format!("{:?}", err),
            Error::ChannelSend(err) => format!("{:?}", err),
            Error::Message(err) => format!("{:?}", err),
            Error::UnknownMessageKey(key) => {
                format!("Unknown message key: {}", key)
            }
            Error::FnInput(err) => format!("{}", err),
            Error::FnOutput(err) => format!("{}", err),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
