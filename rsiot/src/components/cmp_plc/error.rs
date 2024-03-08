use crate::executor::ComponentError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CmpOutput(ComponentError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
