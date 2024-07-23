use serde::{de::DeserializeOwned, Serialize};

use super::{message::Message, MsgData};

use serde_json::{from_str as deserialize, to_string as serialize};

impl<TData> Message<TData>
where
    TData: DeserializeOwned + Serialize,
{
    /// Сериализация сообщений в json
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

    /// Десериализация одного сообщения из json
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

    /// Десериализация массива сообщений из json
    pub fn deserialize_many(text: &str) -> Result<Vec<Self>, super::Error> {
        match deserialize::<Vec<Self>>(text) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                let data = text.to_string();
                Err(super::Error::Deserialization { error, data })
            }
        }
    }
}
