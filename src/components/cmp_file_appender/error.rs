use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | CreateDirAll: {0}")]
    CreateDirAll(std::io::Error),

    #[error("{COMPONENT_NAME} | CreateFile: {0}")]
    CreateFile(std::io::Error),

    #[error("{COMPONENT_NAME} | FlushFile: {0}")]
    FlushFile(std::io::Error),

    #[error("{COMPONENT_NAME} | WriteAll: {0}")]
    WriteAllFile(std::io::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
