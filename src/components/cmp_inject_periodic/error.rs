use super::COMPONENT_NAME;

/// Ошибки компонента cmp_inject_periodic
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | TokioMpscSend: {0}")]
    TokioMpscSend(String),

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for crate::executor::ComponentError {
    fn from(value: Error) -> Self {
        crate::executor::ComponentError::Execution(value.to_string())
    }
}
