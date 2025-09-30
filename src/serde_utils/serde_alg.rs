use serde::{Serialize, de::DeserializeOwned};

use super::{Error, SerdeAlgKind};

/// Алгоритм сериализации / десериализации
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SerdeAlg {
    kind: SerdeAlgKind,
}

impl SerdeAlg {
    /// Создает новый алгоритм сериализации / десериализации
    pub fn new(kind: SerdeAlgKind) -> Self {
        Self { kind }
    }

    /// Сериализация
    pub fn serialize<TData>(
        &self,
        #[allow(unused_variables)] data: &TData,
    ) -> Result<Vec<u8>, Error>
    where
        TData: Serialize,
    {
        #[allow(unreachable_patterns)]
        match self.kind {
            #[cfg(feature = "serde_cbor")]
            SerdeAlgKind::Cbor => super::cbor::serialize(data),

            #[cfg(feature = "serde_json")]
            SerdeAlgKind::Json => super::json::serialize(data),

            #[cfg(feature = "serde_messagepack")]
            SerdeAlgKind::MessagePack => super::messagepack::serialize(data),

            #[cfg(feature = "serde_toml")]
            SerdeAlgKind::Toml => super::toml::serialize(data),

            #[cfg(feature = "serde_postcard")]
            SerdeAlgKind::Postcard => super::postcard::serialize(data),

            SerdeAlgKind::Unspecified => Err(Error::UnknownAlg(self.kind)),
        }
    }

    /// Десериализация
    pub fn deserialize<TData>(&self, #[allow(unused_variables)] data: &[u8]) -> Result<TData, Error>
    where
        TData: DeserializeOwned,
    {
        #[allow(unreachable_patterns)]
        match self.kind {
            #[cfg(feature = "serde_cbor")]
            SerdeAlgKind::Cbor => super::cbor::deserialize(data),

            #[cfg(feature = "serde_json")]
            SerdeAlgKind::Json => super::json::deserialize(data),

            #[cfg(feature = "serde_messagepack")]
            SerdeAlgKind::MessagePack => super::messagepack::deserialize(data),

            #[cfg(feature = "serde_toml")]
            SerdeAlgKind::Toml => super::toml::deserialize(data),

            #[cfg(feature = "serde_postcard")]
            SerdeAlgKind::Postcard => super::postcard::deserialize(data),

            SerdeAlgKind::Unspecified => Err(Error::UnknownAlg(self.kind)),
        }
    }
}
