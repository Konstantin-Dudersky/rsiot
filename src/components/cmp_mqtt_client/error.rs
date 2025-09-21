use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | FnPublish: {0}")]
    FnPublish(anyhow::Error),

    #[error("{COMPONENT_NAME} | FnSubscribe: {0}")]
    FnSubscribe(anyhow::Error),

    #[error("RumqttcClient: {0}")]
    RumqttcClient(#[from] rumqttc::ClientError),

    #[error("{COMPONENT_NAME} | TaskEndInput")]
    TaskEndInput,

    #[error("{COMPONENT_NAME} | TaskEndMain")]
    TaskEndMain,

    #[error("{COMPONENT_NAME} | TaskEndPublish")]
    TaskEndMqttSend,

    #[error("{COMPONENT_NAME} | TaskEndMqttRecv")]
    TaskEndMqttRecv,

    #[error("{COMPONENT_NAME} | TaskEndOutput")]
    TaskEndOutput,

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
