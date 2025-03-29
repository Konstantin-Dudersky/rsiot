use serde::{de::DeserializeOwned, Serialize};

use super::{Error, SerdeAlgKind};

/// Алгоритм сериализации / десериализации
#[derive(Clone, Copy, Debug)]
pub struct SerdeAlg {
    kind: SerdeAlgKind,
}

impl SerdeAlg {
    /// Создает новый алгоритм сериализации / десериализации
    pub fn new(kind: SerdeAlgKind) -> Self {
        Self { kind }
    }

    /// Сериализация
    pub fn serialize<TData>(&self, data: &TData) -> Result<Vec<u8>, Error>
    where
        TData: Serialize,
    {
        #[allow(unreachable_patterns)]
        match self.kind {
            #[cfg(feature = "serde_json")]
            SerdeAlgKind::Json => super::json::serialize(data),

            #[cfg(feature = "serde_toml")]
            SerdeAlgKind::Toml => super::toml::serialize(data),

            _ => Err(Error::UnknownAlg(self.kind)),
        }
    }

    /// Десериализация
    pub fn deserialize<TData>(&self, data: &[u8]) -> Result<TData, Error>
    where
        TData: DeserializeOwned,
    {
        #[allow(unreachable_patterns)]
        match self.kind {
            #[cfg(feature = "serde_json")]
            SerdeAlgKind::Json => super::json::deserialize(data),

            #[cfg(feature = "serde_toml")]
            SerdeAlgKind::Toml => super::toml::deserialize(data),

            _ => Err(Error::UnknownAlg(self.kind)),
        }
    }
}
