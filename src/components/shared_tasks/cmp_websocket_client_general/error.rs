use crate::{components::shared_tasks, executor::ComponentError};

const COMPONENT_NAME: &str = "cmp_websocket_client";

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    BadUrl(#[from] url::ParseError),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Error when setting up connection: {0}")]
    SetupConnection(String),

    #[error(transparent)]
    TaskMsgbusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error("TaskConnectionState")]
    TaskConnectionState,

    #[error("TaskInput")]
    TaskInput,

    #[error("TaskOutput")]
    TaskOutput,

    #[error("TaskSend: {0}")]
    TaskSend(String),

    #[error("TaskReceive: {0}")]
    TaskReceive(String),

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
