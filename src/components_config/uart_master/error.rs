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

    #[error("TaskEndUartComm")]
    TaskEndUartComm,

    #[error("TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error(transparent)]
    TaskFilterIdenticalData(shared_tasks::filter_identical_data::Error),

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus_new::Error),

    #[error("UartRead: {0}")]
    UartRead(String),

    #[error("UartWrite: {0}")]
    UartWrite(String),

    #[error(transparent)]
    TaskMsgbusToBroadcast(shared_tasks::msgbus_to_broadcast::Error),

    #[error("GpioSetup: {0}")]
    GpioSetup(String),

    #[error("GpioPinSet: {0}")]
    GpioPinSet(String),

    #[error("OpenSerialPort: {0}")]
    OpenSerialPort(String),

    #[error(transparent)]
    Device(master_device::Error),

    #[error("BufferFull")]
    BufferFull,
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
