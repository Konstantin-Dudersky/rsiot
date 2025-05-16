/// Ошибки логгирования
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Loki error
    #[cfg(any(
        aarch64_unknown_linux_gnu,
        armv7_unknown_linux_gnueabihf,
        x8664_unknown_linux_gnu
    ))]
    #[error("Loki error: {0}")]
    Loki(#[from] tracing_loki::Error),

    /// Parse error
    #[cfg(any(
        aarch64_unknown_linux_gnu,
        armv7_unknown_linux_gnueabihf,
        x8664_unknown_linux_gnu
    ))]
    #[error("Parse error: {0}")]
    Parse(#[from] url::ParseError),

    #[cfg(riscv32imc_esp_espidf)]
    #[error("Log initialization error: {0}")]
    Log(#[from] esp_idf_svc::sys::EspError),
}
