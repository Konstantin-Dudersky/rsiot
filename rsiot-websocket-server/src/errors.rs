use std::io::Error as StdIoError;
use tokio_tungstenite::tungstenite::Error as WsError;

#[derive(Debug)]
pub enum Errors {
    GetAllFromRedis(String),
    SendToWsError(String),
    MessagesError(String),
    WsError(WsError),
    BindToPortError(StdIoError),
}

impl From<WsError> for Errors {
    fn from(value: WsError) -> Self {
        Self::WsError(value)
    }
}
