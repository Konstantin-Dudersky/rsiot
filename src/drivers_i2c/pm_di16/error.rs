use crate::{components::shared_tasks, serde_utils::postcard_serde};

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    TaskFilterIdenticalData(shared_tasks::filter_identical_data::Error),

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error(transparent)]
    Serde(#[from] postcard_serde::Error),

    #[error("{0}")]
    I2c(String),

    #[error("Tokio mpsc send error: {0}")]
    TokioSyncMpscSender(String),
}
