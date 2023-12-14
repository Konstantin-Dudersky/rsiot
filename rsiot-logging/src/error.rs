#[cfg(any(target_arch = "x86_64"))]
use tracing_loki::{url::ParseError, Error as LokiError};

#[derive(Debug)]
pub enum Error {
    #[cfg(any(target_arch = "x86_64"))]
    LokiError(String),
    #[cfg(any(target_arch = "x86_64"))]
    ParseError(String),
}

#[cfg(any(target_arch = "x86_64"))]
impl From<LokiError> for Error {
    fn from(value: LokiError) -> Self {
        Self::LokiError(value.to_string())
    }
}

#[cfg(any(target_arch = "x86_64"))]
impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value.to_string())
    }
}
