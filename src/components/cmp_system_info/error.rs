use super::COMPONENT_NAME;

/// Ошибки cmp_system_info
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Невозможно определить данные
    #[error("Cannot define: {0}")]
    CannotDefine(String),

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,
}
