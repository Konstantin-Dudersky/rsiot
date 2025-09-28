use crate::{components::shared_tasks, executor::ComponentError, serde_utils};

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Error bind to port: {0}")]
    BindToPort(std::io::Error),

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("TokioSyncMpsc")]
    TokioSyncMpsc,

    #[error("{0}")]
    FnInput(anyhow::Error),

    #[error("Error: {err}, text from client: {data}")]
    FnOutput { err: anyhow::Error, data: String },

    #[error("{COMPONENT_NAME} | FnProcessEnd")]
    FnProcessEnd,

    #[error("Client disconnected")]
    ClientDisconnected,

    #[error(transparent)]
    CmpOutput(crate::executor::ComponentError),

    #[error("TaskEndInput")]
    TaskEndInput,

    #[error("TaskEndOutput")]
    TaskEndOutput,

    #[error(transparent)]
    SharedTaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc_new::Error),

    #[error(transparent)]
    SharedTaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus_new::Error),

    #[error(transparent)]
    Serde(#[from] serde_utils::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
