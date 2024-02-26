use rsiot_messages_core::{Message, MsgDataBound};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct Config<TMsg, TStorageData>
where
    TMsg: MsgDataBound,
    TStorageData: std::fmt::Debug + Default + DeserializeOwned + PartialEq + Serialize,
{
    /// Функция для сохранения информации из входных сообщений в памяти ESP.
    pub fn_input: fn(&TStorageData, &Message<TMsg>) -> Option<TStorageData>,

    /// Функция для выдачи сообщений из сохраненных данных.
    ///
    /// Вызывается один раз, при запуске ESP.
    pub fn_output: fn(&TStorageData) -> Vec<Message<TMsg>>,
}
