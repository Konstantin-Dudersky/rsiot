use crate::{components::shared_tasks, components_config::master_device, executor::ComponentError};

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

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error(transparent)]
    TaskFilter(shared_tasks::filter_identical_data::Error),

    #[error(transparent)]
    TaskMsgbusToBroadcast(shared_tasks::msgbus_to_broadcast::Error),

    #[error(transparent)]
    DeviceError(#[from] master_device::Error),

    #[error("CS number {cs} not availbalve; amount of configured CS: {max_cs}")]
    CsNotAvailable { cs: u8, max_cs: u8 },
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
