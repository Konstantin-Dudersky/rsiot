#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error in fn_process: {0}")]
    FnProcess(anyhow::Error),

    #[error("I2C driver error: {0}")]
    Driver(String),
}
