use tokio::{sync::mpsc::error::SendError, task::JoinError};
use tokio_tungstenite::tungstenite::Error as TungsteniteError;

#[derive(Debug)]
pub enum Error {
    Tungstenite(TungsteniteError),
    JoinError(JoinError),
    SendError(String),
}

impl From<TungsteniteError> for Error {
    fn from(value: TungsteniteError) -> Self {
        Self::Tungstenite(value)
    }
}

impl From<JoinError> for Error {
    fn from(value: JoinError) -> Self {
        Self::JoinError(value)
    }
}

impl<TMessage> From<SendError<TMessage>> for Error {
    fn from(value: SendError<TMessage>) -> Self {
        Self::SendError(value.to_string())
    }
}
