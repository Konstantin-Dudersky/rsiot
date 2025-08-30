use crate::{components::shared_tasks, executor::ComponentError};

/// Ошибки cmp_influxdb
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cmp_influxdb | Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("cmp_influxdb | Status: {status}, message: {message}")]
    RequestParameters {
        /// status
        status: reqwest::StatusCode,
        /// message
        message: String,
    },

    #[error("cmp_influxdb | Cannot represent timetamp as Unix time: {0:?}")]
    WrongTimestamp(crate::message::Timestamp),

    #[error("cmp_influxdb | {0}")]
    Config(#[from] crate::components_config::influxdb3::Error),

    #[error("cmp_influxdb | {0}")]
    TaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error("cmp_influxdb | TaskInputEnd")]
    TaskInputEnd,

    #[error("cmp_influxdb | TaskPeriodicEnd")]
    TaskPeriodicEnd,

    #[error("cmp_influxdb | TaskSendToDatabase")]
    TaskSendToDatabase,

    #[error("cmp_influxdb | TokioMpsc")]
    TokioMpsc,

    #[error("cmp_influxdb | TokioJoin")]
    TokioJoin(#[from] tokio::task::JoinError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
