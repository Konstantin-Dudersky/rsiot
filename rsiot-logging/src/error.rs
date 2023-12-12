use tracing_loki::{url::ParseError, Error as LokiError};

#[derive(Debug)]
pub enum Error {
    LokiError(String),
    ParseError(String),
}

impl From<LokiError> for Error {
    fn from(value: LokiError) -> Self {
        Self::LokiError(value.to_string())
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value.to_string())
    }
}
