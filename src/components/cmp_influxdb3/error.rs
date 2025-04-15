use crate::{components::shared_tasks, executor::ComponentError};

/// Ошибки cmp_influxdb
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// RequestParameters
    #[error("Status: {status}, message: {message}")]
    RequestParameters {
        /// status
        status: reqwest::StatusCode,
        /// message
        message: String,
    },

    /// WrongTimestamp
    #[error("Cannot represent timetamp as Unix time: {0:?}")]
    WrongTimestamp(crate::message::Timestamp),

    /// Config
    #[error(transparent)]
    Config(#[from] crate::components_config::influxdb_v2::Error),

    #[error(transparent)]
    TaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error("TaskInputEnd")]
    TaskInputEnd,

    #[error("TaskPeriodicEnd")]
    TaskPeriodicEnd,

    #[error("TaskSendToDatabase")]
    TaskSendToDatabase,

    #[error("TokioMpsc")]
    TokioMpsc,

    #[error("TokioJoin")]
    TokioJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
