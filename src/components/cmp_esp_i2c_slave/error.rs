use crate::{drivers_i2c::postcard_serde, executor::ComponentError};

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
    Postcard(#[from] postcard_serde::Error),

    #[error("Error in fn_i2c_comm function: {0}")]
    FnI2cComm(anyhow::Error),

    #[error("Error writing to I2C buffer: {0}")]
    WritingToI2cBuffer(esp_idf_hal::sys::EspError),

    #[error("Error reading from I2C buffer: {0}")]
    ReadingFromI2cBuffer(esp_idf_hal::sys::EspError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
