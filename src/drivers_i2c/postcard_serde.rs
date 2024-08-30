//! Сериализация / десериализация, используя Postcard - https://postcard.jamesmunns.com/intro

use std::fmt::Debug;

use postcard::{from_bytes_cobs, to_stdvec_cobs};
use serde::{de::DeserializeOwned, Serialize};

/// Длина сообщения
pub const MESSAGE_LEN: usize = 8;

/// Сериализация данных в формат Postcars
pub fn serialize<T>(data: &T) -> Result<Vec<u8>, Error>
where
    T: Debug + Serialize,
{
    let mut buffer = to_stdvec_cobs(data).map_err(Error::SerializationError)?;

    if buffer.len() > MESSAGE_LEN {
        return Err(Error::BufferTooLarge {
            buffer_len: buffer.len(),
        });
    }
    buffer.resize(MESSAGE_LEN, 0xFF);

    Ok(buffer)
}

/// Десериализация данных из формата Postcard
pub fn deserialize<T>(buffer: &mut [u8]) -> Result<T, Error>
where
    T: Debug + DeserializeOwned,
{
    from_bytes_cobs(buffer).map_err(Error::DeserializationError)
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SerializationError(postcard::Error),

    #[error("Buffer too large. Buffer len: {buffer_len}, message len: {MESSAGE_LEN}. Increase MESSAGE_LEN constant.")]
    BufferTooLarge { buffer_len: usize },

    #[error(transparent)]
    DeserializationError(postcard::Error),
}
