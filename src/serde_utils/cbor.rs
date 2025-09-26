use ciborium::{from_reader, into_writer};
use serde::{Serialize, de::DeserializeOwned};

use super::Error;

pub fn serialize<TData>(data: &TData) -> Result<Vec<u8>, Error>
where
    TData: Serialize,
{
    let mut result = vec![];
    into_writer(data, &mut result).map_err(|e| Error::SerializationError(e.to_string()))?;
    Ok(result)
}

pub fn deserialize<TData>(data: &[u8]) -> Result<TData, Error>
where
    TData: DeserializeOwned,
{
    from_reader(data).map_err(|e| Error::DeserializationError(e.to_string()))
}
