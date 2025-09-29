use crate::{components::shared_tasks, components_config::master_device, executor::ComponentError};

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CmpOutput: {0}")]
    CmpOutput(ComponentError),

    #[error("FnOutput: {0}")]
    FnInput(anyhow::Error),

    #[error("FnOutput: {0}")]
    FnOutput(anyhow::Error),

    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("TokioSyncMpsc")]
    TokioSyncMpsc,

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error(transparent)]
    TaskFilter(shared_tasks::filter_identical_data::Error),

    #[error(transparent)]
    TaskMsgbusToBroadcast(shared_tasks::msgbus_to_broadcast::Error),

    #[error(transparent)]
    DeviceError(#[from] master_device::Error),

    #[error("{COMPONENT_NAME} | CS number {cs} not availbalve; amount of configured CS: {max_cs}")]
    CsNotAvailable { cs: u8, max_cs: u8 },

    #[error("{COMPONENT_NAME} | GpioSetup: {0}")]
    GpioSetup(linux_embedded_hal::gpio_cdev::Error),

    #[error("{COMPONENT_NAME} | GpioPinSet: {0}")]
    GpioPinSet(linux_embedded_hal::gpio_cdev::Error),

    #[error("{COMPONENT_NAME} | SpidevConfigure: {0}")]
    SpidevConfigure(std::io::Error),

    #[error("{COMPONENT_NAME} | SpidevOpen: {0}")]
    SpidevOpen(std::io::Error),

    #[error("{COMPONENT_NAME} | SpidevTransfer: {0}")]
    SpidevTransfer(std::io::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
