#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Message deserialization\nError: {error}\nData: {data}")]
    Deserialization { error: String, data: String },

    #[error("Message serialization error: {0}")]
    Serialization(String),
}
