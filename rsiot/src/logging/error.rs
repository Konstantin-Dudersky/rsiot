/// Ошибки логгирования
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Loki error
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    #[error("Loki error: {0}")]
    Loki(#[from] tracing_loki::Error),

    /// Parse error
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    #[error("Parse error: {0}")]
    Parse(#[from] url::ParseError),
}
