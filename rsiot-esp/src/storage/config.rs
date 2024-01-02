use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct Config<TMessage, TStorageData>
where
    TStorageData: Default + DeserializeOwned + Serialize,
{
    /// Функция для сохранения информации из входных сообщений в памяти ESP.
    pub fn_input: fn(&TStorageData, &TMessage) -> Option<TStorageData>,

    /// Функция для выдачи сообщений из сохраненных данных.
    ///
    /// Вызывается один раз, при запуске ESP.
    pub fn_output: fn(&TStorageData) -> Vec<TMessage>,
}
