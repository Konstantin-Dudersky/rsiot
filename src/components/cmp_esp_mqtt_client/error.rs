use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | CreateEspAsyncMqttClient: {0}")]
    CreateEspAsyncMqttClient(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | FnPublish: {0}")]
    FnPublish(anyhow::Error),

    #[error("{COMPONENT_NAME} | FnSubscribe: {0}")]
    FnSubscribe(anyhow::Error),

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

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
