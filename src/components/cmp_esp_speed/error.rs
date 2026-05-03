use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | CreatePinDriver: {0}")]
    CreatePinDriver(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | SetGpioOutput: {0}")]
    SetGpioOutput(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | TaskFnProcessEnd")]
    TaskFnProcessEnd,

    #[error("{COMPONENT_NAME} | TaskEndCalculate")]
    TaskEndCalculate,

    #[error("{COMPONENT_NAME} | TaskEndEdgeDetect")]
    TaskEndEdgeDetect,

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
