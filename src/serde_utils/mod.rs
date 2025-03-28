//! Утилиты для сериализации / десериализации

#![allow(unreachable_patterns)]

use serde::{de::DeserializeOwned, Serialize};

pub mod postcard_serde;

/// Формат сериализации / десериализации
#[derive(Clone, Debug)]
pub enum SerdeAlgKind {
    /// JSON
    Json,
    /// TOML
    Toml,
}

/// Алгоритм сериализации / десериализации
#[derive(Clone, Debug)]
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
        match self.kind {
            #[cfg(feature = "serde-json")]
            SerdeAlgKind::Json => serialize_json(data),

            #[cfg(feature = "serde-toml")]
            SerdeAlgKind::Toml => serialize_toml(data),

            _ => panic!(
                "Unknown serde algorithm: {:?}. Activate crate feature serde-*",
                self.kind
            ),
        }
    }

    /// Десериализация
    pub fn deserialize<TData>(&self, data: &[u8]) -> Result<TData, Error>
    where
        TData: DeserializeOwned,
    {
        match self.kind {
            #[cfg(feature = "serde-json")]
            SerdeAlgKind::Json => deserialize_json(data),

            #[cfg(feature = "serde-toml")]
            SerdeAlgKind::Toml => deserialize_toml(data),

            _ => panic!(
                "Unknown serde algorithm: {:?}. Activate crate feature serde-*",
                self.kind
            ),
        }
    }
}

#[cfg(feature = "serde-json")]
fn serialize_json<TData>(data: &TData) -> Result<Vec<u8>, Error>
where
    TData: Serialize,
{
    serde_json::to_vec(data).map_err(|e| Error::SerializationError(e.to_string()))
}

#[cfg(feature = "serde-json")]
fn deserialize_json<TData>(data: &[u8]) -> Result<TData, Error>
where
    TData: DeserializeOwned,
{
    serde_json::from_slice(data).map_err(|e| Error::DeserializationError(e.to_string()))
}

#[cfg(feature = "serde-toml")]
fn serialize_toml<TData>(data: &TData) -> Result<Vec<u8>, Error>
where
    TData: Serialize,
{
    let s = toml::to_string(data).map_err(|e| Error::SerializationError(e.to_string()))?;
    Ok(s.as_bytes().to_vec())
}

#[cfg(feature = "serde-toml")]
fn deserialize_toml<TData>(data: &[u8]) -> Result<TData, Error>
where
    TData: DeserializeOwned,
{
    let s = String::from_utf8_lossy(data);
    toml::from_str(&s).map_err(|e| Error::DeserializationError(e.to_string()))
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}
