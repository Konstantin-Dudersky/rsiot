//! Задача перенаправления сообщений из `MsgBusLinker` в  канал `mpsc`

use tokio::{sync::mpsc::Sender, time::error};

use crate::{
    executor::{MsgBusInput, MsgBusLinker},
    message::{Message, MsgDataBound},
};

/// Задача перенаправления сообщений из `MsgBusLinker` в  канал `mpsc`
pub struct MsgBusToMpsc<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Входящий поток сообщений из входа компонента
    pub input: MsgBusInput<TMsg>,

    /// Исходящий поток сообщений
    pub output: Sender<Message<TMsg>>,
}

impl<TMsg> MsgBusToMpsc<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Ok(msg) = self.input.recv().await {
            self.output
                .send(msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }
        Ok(())
    }
}

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("MsgBusToMpsc | TokioSyncMpscSend")]
    TokioSyncMpscSend,
}
