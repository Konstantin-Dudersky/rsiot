use super::COMPONENT_NAME;

/// Ошибки компонента cmp_add_output_stream
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,
}

impl From<Error> for crate::executor::ComponentError {
    fn from(value: Error) -> Self {
        crate::executor::ComponentError::Execution(value.to_string())
    }
}
