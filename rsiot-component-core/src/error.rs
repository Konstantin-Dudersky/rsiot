#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Input not set for component")]
    InputNotSet,

    #[error("Output not set for component")]
    OutputNotSet,

    #[error("Config not set for component")]
    ConfigNotSet,

    #[error("Cache not set for component")]
    CacheNotSet,

    #[error("Function not set for component")]
    FunctionNotSet,
}
