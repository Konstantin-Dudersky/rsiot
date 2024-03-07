#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("{0}")]
    GlooNet(#[from] gloo::net::Error),

    #[error("Error when process on_success callback: {0}")]
    OnSuccess(anyhow::Error),

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    CmpOutput(rsiot_component_core::ComponentError),
}
