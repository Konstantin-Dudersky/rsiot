use esp_idf_svc::sys::EspError;

use crate::executor::ComponentError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] postcard::Error),

    #[error("Read from ESP: {0}")]
    ReadFromEsp(EspError),

    #[error("Save to ESP: {0}")]
    SaveToEsp(EspError),

    #[error("Send to channel error: {0}")]
    SendChannel(String),

    #[error("Take partition error: {0}")]
    TakePartition(EspError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
