use crate::{components::shared_tasks, drivers_i2c::postcard_serde};

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TaskEndInput")]
    TaskInput,

    #[error("TaskI2cComm")]
    TaskI2cComm,

    #[error(transparent)]
    TaskMsgBusToMpsc(#[from] shared_tasks::msg_bus_to_mpsc::Error),

    #[error("TaskOutput")]
    TaskOutput,

    #[error("TokioTaskSend")]
    TokioTaskSend,

    #[error(transparent)]
    PostcardSerde(#[from] postcard_serde::Error),

    #[error("I2c error: {0}")]
    I2c(String),
}
