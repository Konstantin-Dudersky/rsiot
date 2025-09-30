//! Задача перенаправления сообщений из канала `mpsc` в `MsgBusLinker`

use tokio::sync::mpsc;

use crate::{
    executor::{ComponentError, MsgBusLinker, MsgBusOutput},
    message::{Message, MsgDataBound},
};

/// Задача перенаправления сообщений из канала `mpsc` в `MsgBusLinker`
pub struct MpscToMsgBus<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Входящие сообщения
    pub input: mpsc::Receiver<Message<TMsg>>,

    /// Исходящие сообщения, шина сообщений между компонентами
    pub output: MsgBusOutput<TMsg>,
}

impl<TMsg> MpscToMsgBus<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            self.output.send(msg).await?;
        }
        Ok(())
    }
}

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Component(#[from] ComponentError),
}
