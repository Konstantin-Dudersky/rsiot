use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str as from_json, to_string as to_json};

use crate::Error;

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

    /// Десериализация из строки json
    fn from_json(message: &str) -> Result<Self, Error> {
        match from_json::<Self>(message) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(Error::Deserialization(error))
            }
        }
    }

    /// Сериализация в строку json
    fn to_json(&self) -> Result<String, Error> {
        match to_json::<Self>(&self) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(Error::Serialization(error))
            }
        }
    }
}
