#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Component: {0}")]
    Component(#[from] rsiot_component_core::ComponentError),

    #[error("FnInput: {0}")]
    FnInput(anyhow::Error),

    #[error("FnInput: {0}")]
    FnOutput(anyhow::Error),

    #[error("Storage: {0}")]
    Storage(#[from] gloo::storage::errors::StorageError),
}