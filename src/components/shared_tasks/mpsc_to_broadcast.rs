//! Задача перенаправления сообщений из канала `mpsc` в `broadcast`

use tokio::sync::{broadcast, mpsc};

use crate::{
    executor::{MsgBusLinker, ComponentError},
    message::{Message, MsgDataBound},
};

/// Задача перенаправления сообщений из канала `mpsc` в `broadcast`
pub struct Task<T> {
    /// Входящие сообщения
    pub input: mpsc::Receiver<T>,

    /// Исходящие сообщения
    pub output: broadcast::Sender<T>,
}

impl<T> Task<T> {
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            self.output.send(msg).map_err(|_| Error::BroadcastSend)?;
        }
        Ok(())
    }
}

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Component(#[from] ComponentError),

    #[error("BroadcastSend")]
    BroadcastSend,
}
