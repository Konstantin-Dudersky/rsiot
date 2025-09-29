use crate::{components::shared_tasks, components_config::master_device, executor::ComponentError};

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | CmpOutput: {0}")]
    CmpOutput(ComponentError),

    #[error("{COMPONENT_NAME} | FnInput: {0}")]
    FnInput(anyhow::Error),

    #[error("{COMPONENT_NAME} | FnOutput: {0}")]
    FnOutput(anyhow::Error),

    #[error("{COMPONENT_NAME} | DeviceError: {0}")]
    DeviceError(#[from] master_device::Error),

    #[error("{COMPONENT_NAME} | TaskMpscToMsgBus: {0}")]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error("{COMPONENT_NAME} | TaskFilter: {0}")]
    TaskFilter(shared_tasks::filter_identical_data::Error),

    #[error("{COMPONENT_NAME} | TaskMsgbusToBroadcast: {0}")]
    TaskMsgbusToBroadcast(shared_tasks::msgbus_to_broadcast::Error),

    #[error("{COMPONENT_NAME} | TokioSyncMpsc")]
    TokioSyncMpsc,

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | LinuxI2CBusError: {0}")]
    LinuxI2CBusError(#[from] linux_embedded_hal::i2cdev::linux::LinuxI2CError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
