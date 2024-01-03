#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Message deserialization error: {0}")]
    Deserialization(String),
    #[error("Message serialization error: {0}")]
    Serialization(String),
}
