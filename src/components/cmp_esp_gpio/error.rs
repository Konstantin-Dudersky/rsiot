use crate::executor::ComponentError;

use super::COMPONENT_NAME;

/// Ошибки cmp_esp_gpio
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | CreatePinDriver: {0}")]
    CreatePinDriver(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | SetGpioOutput: {0}")]
    SetGpioOutput(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | SetPinPull: {0}")]
    SetPinPull(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | TaskEndGpioOutput")]
    TaskEndGpioOutput,

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | WaitForAnyEdge: {0}")]
    WaitForAnyEdge(esp_idf_svc::sys::EspError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
