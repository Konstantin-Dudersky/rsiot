#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Status: {status}, message: {message}")]
    RequestParameters {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("Cannot represent timetamp as Unix time: {0:?}")]
    WrongTimestamp(rsiot_messages_core::Timestamp),

    #[error(transparent)]
    Config(#[from] rsiot_components_config::influxdb_v2::Error),
}
