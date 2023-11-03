use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str as deserialize, to_string as serialize};

use crate::Errors;

pub trait IMessage
where
    Self: Clone + Debug + DeserializeOwned + Send + Serialize,
{
    /// Ключ для сохранения в базе данных
    fn key(&self) -> String {
        let full_str = format!("{:?}", self);
        let parenth_index = full_str.find('(');
        let full_str: String = match parenth_index {
            Some(value) => full_str.chars().take(value).collect(),
            None => full_str,
        };
        full_str
    }

    fn deser(message: &str) -> Result<Self, Errors> {
        match deserialize::<Self>(message) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(Errors::Deserialization(error))
            }
        }
    }

    fn serialize(&self) -> Result<String, Errors> {
        match serialize(&self) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(Errors::Serialization(error))
            }
        }
    }
}
