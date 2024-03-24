use crate::executor::ComponentError;

/// Ошибки cmp_esp_wifi
#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
