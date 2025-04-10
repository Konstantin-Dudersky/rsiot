use crate::serde_utils;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_utils::Error),

    #[error("CRC mismatch")]
    CrcMismatch,
}
