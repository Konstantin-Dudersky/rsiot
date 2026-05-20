use super::COMPONENT_NAME;

/// Ошибки cmp_derive
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | TaskEndInput")]
    TaskEndInput,

    #[error("{COMPONENT_NAME} | TaskEndOutput")]
    TaskEndOutput,

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),
}
