use std::io::Error as StdIoError;

use tokio::{sync::mpsc::error::SendError, task::JoinError};

#[derive(Debug)]
pub enum Errors {
    Request(String),
    Connection(StdIoError),
    JoinError(JoinError),
    SendError(String),
}

impl From<StdIoError> for Errors {
    fn from(value: StdIoError) -> Self {
        Self::Connection(value)
    }
}

impl From<JoinError> for Errors {
    fn from(value: JoinError) -> Self {
        Self::JoinError(value)
    }
}

impl<TMessage> From<SendError<TMessage>> for Errors {
    fn from(value: SendError<TMessage>) -> Self {
        Self::SendError(value.to_string())
    }
}
