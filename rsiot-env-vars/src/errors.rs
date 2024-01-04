#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("{source}")]
    EnvFileLoadError {
        #[from]
        source: dotenvy::Error,
    },

    #[error("{source}")]
    DeserializeError {
        #[from]
        source: envy::Error,
    },

    #[error("{source}")]
    SerializeError {
        #[from]
        source: toml::ser::Error,
    },

    #[error("{source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Error converting to UPPER_CASE: {0}")]
    ToUppercase(String),
}
