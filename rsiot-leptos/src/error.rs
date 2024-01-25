use rsiot_component_core::ComponentError;

#[derive(Debug, thiserror::Error)]
pub enum Error<TMsg> {
    #[error("Tokio sync error: {0}")]
    TokioSyncMpscSend(#[from] tokio::sync::mpsc::error::SendError<TMsg>),

    #[error("Tokio task join error: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}

impl<TMsg> From<Error<TMsg>> for ComponentError {
    fn from(value: Error<TMsg>) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
