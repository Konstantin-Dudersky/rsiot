use std::io::Error as StdIoError;

use tokio::{sync::mpsc::error::SendError, task::JoinError};
use tokio_tungstenite::tungstenite::Error as WsError;

#[derive(Debug)]
pub enum Errors {
    Websocket(WsError),
    BindToPort(StdIoError),
    Join(JoinError),
    SendToChannel(String),
}

impl From<WsError> for Errors {
    fn from(value: WsError) -> Self {
        Self::Websocket(value)
    }
}

impl From<JoinError> for Errors {
    fn from(value: JoinError) -> Self {
        Self::Join(value)
    }
}

impl<TMessage> From<SendError<TMessage>> for Errors {
    fn from(value: SendError<TMessage>) -> Self {
        Self::SendToChannel(value.to_string())
    }
}
