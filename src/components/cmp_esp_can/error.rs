use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | DriverCreate: {0:?}")]
    DriverCreate(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | DriverRecovery: {0:?}")]
    DriverRecovery(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | DriverStart: {0:?}")]
    DriverStart(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | FilterSetup: {0:?}")]
    FilterSetup(&'static str),

    #[error("{COMPONENT_NAME} | FrameConversionFromField: {0}")]
    FrameConversionFromField(String),

    #[error("{COMPONENT_NAME} | FrameConversionIntoField: {0:?}")]
    FrameConversionIntoField(crate::components_config::can_general::CanFrame),

    #[error("{COMPONENT_NAME} | TaskEnd")]
    TaskEnd,

    #[error("{COMPONENT_NAME} | TaskEndInput")]
    TaskEndInput,

    #[error("{COMPONENT_NAME} | TaskEndOutput")]
    TaskEndOutput,

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
