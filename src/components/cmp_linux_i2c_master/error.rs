use crate::{components::shared_tasks, components_config::master_device, executor::ComponentError};

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cmp_linux_i2c_master | CmpOutput: {0}")]
    CmpOutput(ComponentError),

    #[error("cmp_linux_i2c_master | FnInput: {0}")]
    FnInput(anyhow::Error),

    #[error("cmp_linux_i2c_master | FnOutput: {0}")]
    FnOutput(anyhow::Error),

    #[error("cmp_linux_i2c_master | DeviceError: {0}")]
    DeviceError(#[from] master_device::Error),

    #[error("cmp_linux_i2c_master | TaskMpscToMsgBus: {0}")]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error("cmp_linux_i2c_master | TaskFilter: {0}")]
    TaskFilter(shared_tasks::filter_identical_data::Error),

    #[error("cmp_linux_i2c_master | TaskMsgbusToBroadcast: {0}")]
    TaskMsgbusToBroadcast(shared_tasks::msgbus_to_broadcast::Error),

    #[error("cmp_linux_i2c_master | TokioSyncMpsc")]
    TokioSyncMpsc,

    #[error("cmp_linux_i2c_master | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("cmp_linux_i2c_master | LinuxI2CBusError: {0}")]
    LinuxI2CBusError(#[from] linux_embedded_hal::i2cdev::linux::LinuxI2CError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
