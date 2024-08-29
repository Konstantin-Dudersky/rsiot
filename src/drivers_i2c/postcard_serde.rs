use std::fmt::Debug;

use postcard::{from_bytes_cobs, to_stdvec_cobs};
use serde::{de::DeserializeOwned, Serialize};

pub const MESSAGE_LEN: usize = 8;

pub fn serialize<T>(data: &T) -> Result<Vec<u8>, String>
where
    T: Debug + Serialize,
{
    let mut response_buffer = to_stdvec_cobs(data).map_err(|e| format!("{}", e))?;

    if response_buffer.len() > MESSAGE_LEN {
        let err = format!(
            "Response too large. Buffer len: {}, need size: {}",
            MESSAGE_LEN,
            response_buffer.len()
        );
        return Err(err);
    }
    response_buffer.resize(MESSAGE_LEN, 0xFF);
    Ok(response_buffer)
}

pub fn deserialize<T>(request_buffer: &mut [u8]) -> Result<T, String>
where
    T: Debug + DeserializeOwned,
{
    from_bytes_cobs(request_buffer).map_err(|e| format!("{}", e))
}
