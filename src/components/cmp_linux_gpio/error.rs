use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | CreateAsyncLineEventHandle: {0}")]
    CreateAsyncLineEventHandle(linux_embedded_hal::gpio_cdev::Error),

    #[error("{COMPONENT_NAME} | CreateLineEventHandle: {0}")]
    CreateLineEventHandle(linux_embedded_hal::gpio_cdev::Error),

    #[error("{COMPONENT_NAME} | GpioSetup: {0}")]
    GpioSetup(linux_embedded_hal::gpio_cdev::Error),

    #[error("{COMPONENT_NAME} | GpioSetValue: {0}")]
    GpioSetValue(linux_embedded_hal::gpio_cdev::Error),

    #[error("{COMPONENT_NAME} TaskEnd")]
    TaskEnd,

    #[error("{COMPONENT_NAME} TaskGpioInputEnd")]
    TaskGpioInputEnd,

    #[error("{COMPONENT_NAME} TaskGpioOutputEnd")]
    TaskGpioOutputEnd,

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | UnwrapEvent: {0}")]
    UnwrapEvent(linux_embedded_hal::gpio_cdev::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
