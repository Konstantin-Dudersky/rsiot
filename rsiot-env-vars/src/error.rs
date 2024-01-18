#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("Error loading .env file: {0}")]
    LoadFromFile(#[from] dotenvy::Error),

    #[error("Deserialization error: {0}")]
    LoadFromEnvironment(#[from] envy::Error),

    #[error("Serialization error: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("Error converting to UPPER_CASE: {0}")]
    ToUppercase(String),
}
