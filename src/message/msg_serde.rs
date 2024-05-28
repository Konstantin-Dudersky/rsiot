use serde::{de::DeserializeOwned, Serialize};

use super::{message::Message, MsgData};

#[cfg(feature = "serde-json")]
use serde_json::{from_str as deserialize, to_string as serialize};

impl<TData> Message<TData>
where
    TData: DeserializeOwned + Serialize,
{
    #[cfg(not(feature = "serde-json"))]
    pub fn serialize(&self) -> Result<String, crate::Error> {
        let err = "Serialization feature not select".to_string();
        Err(crate::Error::Serialization(err))
    }

    #[cfg(not(feature = "serde-json"))]
    pub fn deserialize(_text: &str) -> Result<Self, crate::Error> {
        let error = "Serialization feature not select".to_string();
        let data = "".to_string();
        Err(crate::Error::Deserialization { error, data })
    }

    /// Сериализация сообщений в json
    #[cfg(feature = "serde-json")]
    pub fn serialize(&self) -> Result<String, super::Error> {
        let json = serialize::<Self>(self);
        match json {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(super::Error::Serialization(error))
            }
        }
    }

    /// Сериализация полей данных сообщений в json
    #[cfg(feature = "serde-json")]
    pub fn serialize_data(&self) -> Result<String, super::Error> {
        let json = serialize::<MsgData<TData>>(&self.data);
        match json {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(super::Error::Serialization(error))
            }
        }
    }

    /// Десериализация сообщений из json
    #[cfg(feature = "serde-json")]
    pub fn deserialize(text: &str) -> Result<Self, super::Error> {
        match deserialize::<Self>(text) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                let data = text.to_string();
                Err(super::Error::Deserialization { error, data })
            }
        }
    }
}
