use crate::{components::shared_tasks, executor::ComponentError};

/// Ошибки cmp_plc
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// CmpOutput
    #[error(transparent)]
    CmpOutput(ComponentError),

    #[error(transparent)]
    FilterMsgsWithSameData(#[from] shared_tasks::filter_identical_data::Error),

    #[error("Internal channel error: {0}")]
    TokioSyncMpsc(String),

    #[error("TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    ToCmpOutput(#[from] shared_tasks::mpsc_to_msg_bus::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
