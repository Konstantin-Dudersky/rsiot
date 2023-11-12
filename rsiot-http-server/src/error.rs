use std::net::AddrParseError;

use axum::{http::StatusCode, response::IntoResponse};

use hyper::Error as HyperError;
use rsiot_messages_core::Error as MessageError;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug)]
pub enum Error<TMessage> {
    UnknownMessageKey(String),
    Message(MessageError),
    ChannelSendError(SendError<TMessage>),
    HyperError(HyperError),
    AddrParseError(AddrParseError),
}

impl<TMessage> From<MessageError> for Error<TMessage> {
    fn from(value: MessageError) -> Self {
        Self::Message(value)
    }
}

impl<TMessage> From<SendError<TMessage>> for Error<TMessage> {
    fn from(value: SendError<TMessage>) -> Self {
        Self::ChannelSendError(value)
    }
}

impl<TMessage> From<HyperError> for Error<TMessage> {
    fn from(value: HyperError) -> Self {
        Self::HyperError(value)
    }
}

impl<TMessage> From<AddrParseError> for Error<TMessage> {
    fn from(value: AddrParseError) -> Self {
        Self::AddrParseError(value)
    }
}

impl<TMessage> IntoResponse for Error<TMessage> {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            Error::UnknownMessageKey(key) => {
                format!("Unknown message key: {}", key)
            }
            Error::Message(err) => format!("{:?}", err),
            Error::ChannelSendError(err) => format!("{:?}", err),
            Error::HyperError(err) => format!("{:?}", err),
            Error::AddrParseError(err) => format!("{:?}", err),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
