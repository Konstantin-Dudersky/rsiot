use super::SerdeAlgKind;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Unknown serde algorithm: {0:?}. Activate crate feature serde_*")]
    UnknownAlg(SerdeAlgKind),
}
