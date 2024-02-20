use super::message::Message;

use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "serde-json")]
use serde_json::{from_str as deserialize, to_string as serialize};

impl<TData> Message<TData>
where
    TData: DeserializeOwned + Serialize,
{
    /// Сериализация сообщений в json
    #[cfg(feature = "serde-json")]
    pub fn serialize(&self) -> Result<String, crate::Error> {
        match serialize::<Self>(self) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(crate::Error::Serialization(error))
            }
        }
    }

    /// Десериализация сообщений из json
    #[cfg(feature = "serde-json")]
    pub fn deserialize(text: &str) -> Result<Self, crate::Error> {
        match deserialize::<Self>(text) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(crate::Error::Deserialization(error))
            }
        }
    }
}
