use crate::executor::ComponentError;

/// Ошибки cmp_http_server_esp
#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
