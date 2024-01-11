use url::ParseError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    #[error("{source}")]
    LokiError {
        #[from]
        source: tracing_loki::Error,
    },

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    #[error("{source}")]
    ParseError {
        #[from]
        source: url::ParseError,
    },
}
