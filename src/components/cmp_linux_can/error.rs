use std::io;

use crate::{components_config::can_general::CanFrame, executor::ComponentError};

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | FrameConversion: {0:?}")]
    FrameConversion(CanFrame),

    #[error("{COMPONENT_NAME} | InterfaceDetails: {0:?}")]
    InterfaceDetails(String),

    #[error("{COMPONENT_NAME} | InterfaceDown: {0:?}")]
    InterfaceDown(String),

    #[error("{COMPONENT_NAME} | InterfaceOpen: {0:?}")]
    InterfaceOpen(String),

    #[error("{COMPONENT_NAME} | InterfaceState: {0:?}")]
    InterfaceState(String),

    #[error("{COMPONENT_NAME} | InterfaceUp: {0:?}")]
    InterfaceUp(String),

    #[error("{COMPONENT_NAME} | InvalidId: {0}")]
    InvalidId(u64),

    #[error("{COMPONENT_NAME} | ProcessExecution: {0}")]
    ProcessExecution(io::Error),

    #[error("{COMPONENT_NAME} | ReadFrame: {0}")]
    ReadFrame(socketcan::Error),

    #[error("{COMPONENT_NAME} | SetFilters: {0}")]
    SetFilters(io::Error),

    #[error("{COMPONENT_NAME} | SocketOpen: {0}")]
    SocketOpen(io::Error),

    #[error("{COMPONENT_NAME} | Sudo: {0}")]
    Sudo(String),

    #[error("{COMPONENT_NAME} | TaskEnd")]
    TaskEnd,

    #[error("{COMPONENT_NAME} | TaskEndInput")]
    TaskEndInput,

    #[error("{COMPONENT_NAME} | TaskEndOutput")]
    TaskEndOutput,

    #[error("{COMPONENT_NAME} | TaskEndRecvFromCan")]
    TaskEndRecvFromCan,

    #[error("{COMPONENT_NAME} | TaskEndSendToCan")]
    TaskEndSendToCan,

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | WriteFrame: {0}")]
    WriteFrame(io::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
