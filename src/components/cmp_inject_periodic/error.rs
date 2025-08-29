/// Ошибки компонента cmp_inject_periodic
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cmp_inject_periodic | TokioMpscSend: {0}")]
    TokioMpscSend(String),

    #[error("cmp_inject_periodic | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for crate::executor::ComponentError {
    fn from(value: Error) -> Self {
        crate::executor::ComponentError::Execution(value.to_string())
    }
}
