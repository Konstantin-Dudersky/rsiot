/// Ошибки cmp_influxdb
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
}
