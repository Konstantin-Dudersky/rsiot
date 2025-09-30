//! Задача перенаправления сообщений из канала `mpsc` в `MsgBusLinker`

use tokio::sync::mpsc;

use crate::{
    executor::{ComponentError, MsgBusLinker},
    message::{Message, MsgDataBound},
};

/// Задача перенаправления сообщений из канала `mpsc` в `MsgBusLinker`
#[deprecated]
pub struct MpscToMsgBus<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Входящие сообщения
    pub input: mpsc::Receiver<Message<TMsg>>,

    /// Исходящие сообщения, шина сообщений между компонентами
    pub msg_bus: MsgBusLinker<TMsg>,
}

impl<TMsg> MpscToMsgBus<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            self.msg_bus.send_output(msg).await?;
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
