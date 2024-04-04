/// Ошибки модуля работы с переменными среды
#[derive(Debug, thiserror::Error)]
pub enum Errors {
    /// LoadFromFile
    #[error("Error loading .env file: {0}")]
    LoadFromFile(#[from] dotenvy::Error),

    /// LoadFromEnvironment
    #[error("Deserialization error: {0}")]
    LoadFromEnvironment(#[from] envy::Error),

    /// SerializeError
    #[error("Serialization error: {0}")]
    SerializeError(#[from] toml::ser::Error),

    /// IoError
    #[error("{0}")]
    IoError(#[from] std::io::Error),

    /// ToUppercase
    #[error("Error converting to UPPER_CASE: {0}")]
    ToUppercase(String),
}
