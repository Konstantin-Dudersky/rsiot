use crate::executor::ComponentError;

/// Ошибки компонента cmp_raspberrypi_gpio
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Ошибка отправки сообщения в исходящий канал
    #[error("CmpOutput: {0}")]
    CmpOutput(crate::executor::ComponentError),

    /// Ошибка доступа к GPIO
    #[error("RppalGpio: {0}")]
    RppalGpio(#[from] rppal::gpio::Error),

    /// Ошибка tokio
    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
