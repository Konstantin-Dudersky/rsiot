//! Сериализация / десериализация, используя Postcard - <https://postcard.jamesmunns.com/intro>

#![deprecated]

use std::fmt::Debug;

use crc::{Crc, Digest, Table, CRC_32_ISCSI};
use postcard::{from_bytes, from_bytes_crc32, to_slice_crc32, to_stdvec, to_stdvec_crc32};
use serde::{de::DeserializeOwned, Serialize};

/// Длина сообщения
pub const MESSAGE_LEN: usize = 32;

const CRC_DIGEST: Digest<u32, Table<1>> = Crc::<u32>::new(&CRC_32_ISCSI).digest();

/// Сериализация данных в формат, в вектор байт, без CRC
pub fn serialize_nocrc<T>(data: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let buffer = to_stdvec(data).map_err(Error::SerializationError)?;
    Ok(buffer)
}

/// Сериализация данных в формат, в вектор байт, с CRC
pub fn serialize_crc<T>(data: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let buffer = to_stdvec_crc32(data, CRC_DIGEST).map_err(Error::SerializationError)?;

    Ok(buffer)
}

/// Десериализация данных из формата Postcard
pub fn deserialize_crc<T>(buffer: &mut [u8]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    from_bytes_crc32(buffer, CRC_DIGEST).map_err(|e| Error::DeserializationError {
        error: e,
        buffer: buffer.to_vec(),
    })
}

/// Десериализация данных из формата Postcard
pub fn deserialize_nocrc<T>(buffer: &mut [u8]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    from_bytes(buffer).map_err(|e| Error::DeserializationError {
        error: e,
        buffer: buffer.to_vec(),
    })
}

/// Сериализация данных в формат Postcard
#[deprecated]
pub fn serialize_nocrc_deprecated<T>(data: &T) -> Result<Vec<u8>, Error>
where
    T: Debug + Serialize,
{
    let mut buffer = to_stdvec(data).map_err(Error::SerializationError)?;

    if buffer.len() > MESSAGE_LEN {
        return Err(Error::BufferTooLarge {
            buffer_len: buffer.len(),
        });
    }
    buffer.resize(MESSAGE_LEN, 0xFF);

    Ok(buffer)
}

/// Сериализация данных в формат Postcard
#[deprecated]
pub fn serialize_crc_deprecated<T>(data: &T) -> Result<Vec<u8>, Error>
where
    T: Debug + Serialize,
{
    let mut buffer = to_stdvec_crc32(data, CRC_DIGEST).map_err(Error::SerializationError)?;

    if buffer.len() > MESSAGE_LEN {
        return Err(Error::BufferTooLarge {
            buffer_len: buffer.len(),
        });
    }
    buffer.resize(MESSAGE_LEN, 0xFF);

    Ok(buffer)
}

/// Сериализация данных в формат Postcard
#[deprecated]
pub fn serialize<T, const MESSAGE_LEN: usize>(data: &T) -> Result<[u8; MESSAGE_LEN], Error>
where
    T: Debug + Serialize,
{
    let mut buffer = [0xFF; MESSAGE_LEN];
    let _ = to_slice_crc32(data, &mut buffer, CRC_DIGEST).map_err(Error::SerializationError)?;

    if buffer.len() > MESSAGE_LEN {
        return Err(Error::BufferTooLarge {
            buffer_len: buffer.len(),
        });
    }

    Ok(buffer)
}

/// Десериализация данных из формата Postcard
#[deprecated]
pub fn deserialize<T>(buffer: &mut [u8]) -> Result<T, Error>
where
    T: Debug + DeserializeOwned,
{
    deserialize_crc(buffer)
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SerializationError(postcard::Error),

    #[error("Buffer too large. Buffer len: {buffer_len}, message len: {MESSAGE_LEN}. Increase MESSAGE_LEN constant.")]
    BufferTooLarge { buffer_len: usize },

    #[error("Deserialization error: {error}. Buffer: {buffer:x?}")]
    DeserializationError {
        error: postcard::Error,
        buffer: Vec<u8>,
    },
}
