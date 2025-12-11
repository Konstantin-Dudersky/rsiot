use linux_embedded_hal::serialport;

use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("BufferFull")]
    BufferFull,

    #[error("{COMPONENT_NAME} | CmpOutput: {0}")]
    CmpOutput(ComponentError),

    #[error("{COMPONENT_NAME} | FnProcessEnd")]
    FnProcessEnd,

    #[error("{COMPONENT_NAME} | FnOutput: {0}")]
    FnOutput(anyhow::Error),

    #[error("{COMPONENT_NAME} | PortClear: {0}")]
    PortClear(serialport::Error),

    #[error("{COMPONENT_NAME} | PortOpen: {0}")]
    PortOpen(serialport::Error),

    #[error("UartRead: {0}")]
    PortRead(String),

    #[error("{COMPONENT_NAME} | TaskEnd")]
    TaskEnd,

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
