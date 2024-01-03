#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("Component initialization error: {0}")]
    Initialization(String),

    #[error("Component execution error")]
    Execution(String),
}
