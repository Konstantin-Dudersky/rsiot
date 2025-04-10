use crc::{Crc, Digest, Table, CRC_32_ISCSI};
use postcard::{from_bytes, to_stdvec};
use serde::{de::DeserializeOwned, Serialize};

use super::Error;

pub fn serialize<TData>(data: &TData) -> Result<Vec<u8>, Error>
where
    TData: Serialize,
{
    to_stdvec(data).map_err(|e| Error::SerializationError(e.to_string()))
}

pub fn deserialize<TData>(data: &[u8]) -> Result<TData, Error>
where
    TData: DeserializeOwned,
{
    from_bytes(data).map_err(|e| Error::DeserializationError(e.to_string()))
}
