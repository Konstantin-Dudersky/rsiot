#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    #[error("{source}")]
    Loki {
        #[from]
        source: tracing_loki::Error,
    },

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    #[error("{source}")]
    Parse {
        #[from]
        source: url::ParseError,
    },
}
