use serde::{de::DeserializeOwned, Serialize};
use toml::{from_str, to_string};

use super::Error;

pub fn serialize<TData>(data: &TData) -> Result<Vec<u8>, Error>
where
    TData: Serialize,
{
    let s = to_string(data).map_err(|e| Error::SerializationError(e.to_string()))?;
    Ok(s.as_bytes().to_vec())
}

pub fn deserialize<TData>(data: &[u8]) -> Result<TData, Error>
where
    TData: DeserializeOwned,
{
    let s = String::from_utf8_lossy(data);
    from_str(&s).map_err(|e| Error::DeserializationError(e.to_string()))
}
