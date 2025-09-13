//! Фильтрация сообщений с одинаковым полем data.

use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::{
    executor::CheckCapacity,
    message::{Message, MsgDataBound},
};

const COMPONENT_NAME: &str = "filter_identical_data";

/// Фильтрация сообщений с одинаковым полем data.
///
/// Функция fn_output генерирует сообщения со скоростью цикла ПЛК. Большинство сообщений с
/// одинаковым полем `data`, но с разными метками времени. Данная функция сохраняет все сообщения в кеше, и отдает только с обноволенным полем data.
///
/// TODO - возможно, все-таки периодически выдавать сообщения, даже если поле `data` не изменилось
pub struct FilterIdenticalData<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Входящие сообщения
    pub input: mpsc::Receiver<Message<TMsg>>,

    /// Исходящие сообщения
    pub output: mpsc::Sender<Message<TMsg>>,
}

impl<TMsg> FilterIdenticalData<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut cache: HashMap<String, Message<TMsg>> = HashMap::new();

        while let Some(msg) = self.input.recv().await {
            let key = &msg.key;

            let msg_cache = cache.get(key);

            // Если сообщения нет в кеше, сохраняем в кеш и отдаем на выход
            let msg_cache = match msg_cache {
                Some(val) => val,
                None => {
                    cache.insert(key.to_string(), msg.clone());
                    self.output
                        .check_capacity(0.2, "filter_identical_data")
                        .send(msg)
                        .await
                        .map_err(|_| Error::TokioSyncMpscSend)?;
                    continue;
                }
            };

            // Если поле `data` совпадает, пропускаем сообщение
            if msg_cache.data == msg.data {
                continue;
            }

            // Сообщение новое, сохраняем в кеш и отдаем на выход
            cache.insert(key.to_string(), msg.clone());
            self.output
                .check_capacity(0.2, "filter_identical_data")
                .send(msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }

        Err(Error::TaskEnd)
    }
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{COMPONENT_NAME} | TaskEnd")]
    TaskEnd,
}
