/// Ошибки логгирования
#[allow(missing_docs)]
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

    #[cfg(riscv32imc_esp_espidf)]
    #[error("Log initialization error: {0}")]
    Log(#[from] esp_idf_svc::sys::EspError),
}
