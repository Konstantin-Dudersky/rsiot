use std::io;

use crate::{components_config::can_general::Frame, executor::ComponentError};

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | FrameConversion: {0:?}")]
    FrameConversion(Frame),

    #[error("{COMPONENT_NAME} | InvalidId: {0}")]
    InvalidId(u64),

    #[error("{COMPONENT_NAME} | SocketOpen: {0}")]
    SocketOpen(io::Error),

    #[error("{COMPONENT_NAME} | TaskEnd")]
    TaskEnd,

    #[error("{COMPONENT_NAME} | TaskEndInput")]
    TaskEndInput,

    #[error("{COMPONENT_NAME} | TaskEndOutput")]
    TaskEndOutput,

    #[error("{COMPONENT_NAME} | TaskEndSendToCan")]
    TaskEndSendToCan,

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
