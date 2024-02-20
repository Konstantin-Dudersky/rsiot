#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("Component initialization error: {0}")]
    Initialization(String),

    #[error("Component execution error: {0}")]
    Execution(String),

    #[error("Component output error: {0}")]
    CmpInput(String),

    #[error("Component output error: {0}")]
    CmpOutput(String),
}
