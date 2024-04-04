use hmac::digest::InvalidLength;

/// Ошибки cmp_auth
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// TokioTaskJoinError
    #[error("TokioTaskJoinError: {0}")]
    TokioTaskJoinError(#[from] tokio::task::JoinError),

    /// CmpOutput
    #[error("CmpOutput: {0}")]
    CmpOutput(crate::executor::ComponentError),

    /// ProcessRequest
    #[error("ProcessRequest: {0}")]
    ProcessRequest(String),

    /// Hmac error
    #[error("Hmac error: {0}")]
    Hmac(#[from] InvalidLength),

    /// Jwt error
    #[error("Jwt error: {0}")]
    Jwt(#[from] jwt::Error),
}
