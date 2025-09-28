use crate::{components::shared_tasks, components_config::master_device, executor::ComponentError};

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CmpOutput: {0}")]
    CmpOutput(ComponentError),

    #[error("FnOutput: {0}")]
    FnOutput(anyhow::Error),

    #[error("{COMPONENT_NAME} | DeviceError: {0}")]
    DeviceError(#[from] master_device::Error),

    #[error("{COMPONENT_NAME} | FnProcessEnd")]
    FnProcessEnd,

    #[error("{COMPONENT_NAME} | I2cDriverCreation: {0}")]
    I2cDriverCreation(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | TaskEndI2cComm")]
    TaskEndI2cComm,

    #[error("{COMPONENT_NAME} | TaskFilter: {0}")]
    TaskFilter(shared_tasks::filter_identical_data::Error),

    #[error("{COMPONENT_NAME} | TaskMpscToMsgBus: {0}")]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus_new::Error),

    #[error("{COMPONENT_NAME} | TaskMsgbusToBroadcast: {0}")]
    TaskMsgbusToBroadcast(shared_tasks::msgbus_to_broadcast::Error),

    #[error("{COMPONENT_NAME} | TokioSyncMpsc")]
    TokioSyncMpsc,

    #[error("{COMPONENT_NAME}: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
