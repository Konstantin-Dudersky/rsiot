use crate::{components::shared_tasks, components_config::master_device, executor::ComponentError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CmpOutput(crate::executor::ComponentError),

    #[error("{0}")]
    Connection(#[from] std::io::Error),

    #[error(transparent)]
    Device(#[from] master_device::Error),

    // #[error("Modbus request error. Request: {request:?}. Error: {error}")]
    // Request {
    //     request: super::config::Request,
    //     error: String,
    // },
    #[error(transparent)]
    TaskFilter(shared_tasks::filter_identical_data::Error),

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error(transparent)]
    TaskMsgbusToBroadcast(shared_tasks::msgbus_to_broadcast::Error),

    #[error(transparent)]
    TokioTimeout(#[from] tokio::time::error::Elapsed),

    #[error("TokioSyncMpsc")]
    TokioSyncMpsc,

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
