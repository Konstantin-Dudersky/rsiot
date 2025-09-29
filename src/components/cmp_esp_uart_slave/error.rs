use crate::{components::shared_tasks, executor::ComponentError};

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | CmpOutput: {0}")]
    CmpOutput(ComponentError),

    #[error("{COMPONENT_NAME} | CreateAsyncUartDriver: {0}")]
    CreateAsyncUartDriver(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | FnOutput: {0}")]
    FnInput(anyhow::Error),

    #[error("{COMPONENT_NAME} | FnOutput: {0}")]
    FnOutput(anyhow::Error),

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | FnUartComm: {0}")]
    FnUartComm(anyhow::Error),

    #[error("{COMPONENT_NAME} | TaskOutput: {0}")]
    TaskOutput(String),

    #[error("{COMPONENT_NAME} | TaskFilterIdenticalData: {0}")]
    TaskFilterIdenticalData(shared_tasks::filter_identical_data::Error),

    #[error("{COMPONENT_NAME} | TaskMpscToMsgbus: {0}")]
    TaskMpscToMsgbus(shared_tasks::mpsc_to_msgbus::Error),

    #[error("{COMPONENT_NAME} | UartFlush: {0}")]
    UartFlush(esp_idf_svc::io::EspIOError),

    #[error("{COMPONENT_NAME} | UartWriteAll: {0}")]
    UartWriteAll(esp_idf_svc::io::EspIOError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
