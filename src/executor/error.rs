/// Ошибки исполненителя компонентов
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    /// Component initialization error
    #[error("Component initialization error: {0}")]
    Initialization(String),

    /// Component execution error
    #[error("Component execution error: {0}")]
    Execution(String),

    /// Component input error
    #[error("Component input error: {0}")]
    CmpInput(String),

    /// Component output error
    #[error("Component output error: {0}")]
    CmpOutput(String),

    #[error("CmpExecutor | TaskInternalSend")]
    TaskInternalSend,
}
