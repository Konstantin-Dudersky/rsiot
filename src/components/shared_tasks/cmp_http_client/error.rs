use crate::{components::shared_tasks, executor::ComponentError};

const COMPONENT_NAME: &str = "cmp_http_client";

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | TaskInputRequest")]
    TaskInputRequest,

    #[error("{COMPONENT_NAME} | TaskPeriodicRequest")]
    TaskPeriodicRequest,

    #[error("{COMPONENT_NAME} | TaskProcessResponse")]
    TaskProcessResponse,

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{COMPONENT_NAME} | TokioJoin")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | TaskMsgBusToMpsc: {0}")]
    TaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error("{COMPONENT_NAME} | TaskMpscToMsgBus: {0}")]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error("{COMPONENT_NAME} | End execution of HTTP client: {0}")]
    TaskEndHttpClient(String),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
