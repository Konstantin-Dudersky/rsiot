/// Ошибки логгирования
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Loki error
    #[cfg(feature = "log_loki")]
    #[error("Loki error: {0}")]
    Loki(#[from] tracing_loki::Error),

    /// Parse error
    #[cfg(feature = "log_loki")]
    #[error("Parse error: {0}")]
    Parse(#[from] url::ParseError),

    #[cfg(feature = "log_esp")]
    #[error("Log initialization error: {0}")]
    Log(#[from] esp_idf_svc::sys::EspError),

    #[error("Failed to read RUST_LOG environment variable: {0}")]
    Env(#[from] std::env::VarError),
}
