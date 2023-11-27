use dotenvy::Error as DotenvyError;
use envy::Error as EnvyError;
use std::io::Error as IoError;
use toml::ser::Error as TomlError;

#[derive(Debug)]
pub enum Errors {
    EnvFileLoadError(String),
    DeserializeError(String),
    SerializeError(String),
    IoError(String),
    /// Ошибка преобразования переменных в верхний регистр
    ToUppercase(String),
}

impl From<DotenvyError> for Errors {
    fn from(value: DotenvyError) -> Self {
        Self::EnvFileLoadError(value.to_string())
    }
}

impl From<EnvyError> for Errors {
    fn from(value: EnvyError) -> Self {
        Self::DeserializeError(value.to_string())
    }
}

impl From<TomlError> for Errors {
    fn from(value: TomlError) -> Self {
        Self::SerializeError(value.to_string())
    }
}

impl From<IoError> for Errors {
    fn from(value: IoError) -> Self {
        Self::IoError(value.to_string())
    }
}
