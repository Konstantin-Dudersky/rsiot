use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | BlockingStdIo: {0}")]
    BlockingStdIo(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | CreateUsbSerialDriver: {0}")]
    CreateUsbSerialDriver(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | GpioWaitForEdge: {0}")]
    GpioWaitForEdge(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | PinDriver: {0}")]
    PinDriver(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | StdoutFlush: {0}")]
    StdoutFlush(std::io::Error),

    #[error("{COMPONENT_NAME} | StdinRead: {0}")]
    StdinRead(std::io::Error),

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
