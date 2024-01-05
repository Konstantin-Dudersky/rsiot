use url::ParseError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(target_arch = "x86_64")]
    #[error("{source}")]
    LokiError {
        #[from]
        source: tracing_loki::Error,
    },

    #[cfg(target_arch = "x86_64")]
    #[error("{source}")]
    ParseError {
        #[from]
        source: url::ParseError,
    },
}
