use hmac::digest::InvalidLength;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TokioTaskJoinError: {0}")]
    TokioTaskJoinError(#[from] tokio::task::JoinError),

    #[error("CmpOutput: {0}")]
    CmpOutput(crate::executor::ComponentError),

    #[error("ProcessRequest: {0}")]
    ProcessRequest(String),

    #[error("Hmac error: {0}")]
    Hmac(#[from] InvalidLength),

    #[error("Jwt error: {0}")]
    Jwt(#[from] jwt::Error),
}
