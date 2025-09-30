use rmp_serde::{from_slice, to_vec};
use serde::{Serialize, de::DeserializeOwned};

use super::Error;

pub fn serialize<TData>(data: &TData) -> Result<Vec<u8>, Error>
where
    TData: Serialize,
{
    to_vec(data).map_err(|e| Error::SerializationError(e.to_string()))
}

pub fn deserialize<TData>(data: &[u8]) -> Result<TData, Error>
where
    TData: DeserializeOwned,
{
    from_slice(data).map_err(|e| Error::DeserializationError(e.to_string()))
}
