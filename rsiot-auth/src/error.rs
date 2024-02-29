#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TokioTaskJoinError: {0}")]
    TokioTaskJoinError(#[from] tokio::task::JoinError),

    #[error("CmpOutput: {0}")]
    CmpOutput(rsiot_component_core::ComponentError),
}
