use std::io::Error as StdIoError;

#[derive(Debug)]
pub enum Errors {
    Read(String),
    Write(String),
    ChannelSendError(String),
    Connection(StdIoError),
}

impl From<StdIoError> for Errors {
    fn from(value: StdIoError) -> Self {
        Self::Connection(value)
    }
}
