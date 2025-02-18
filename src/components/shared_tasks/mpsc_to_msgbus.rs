//! Задача перенаправления сообщений из канала `mpsc` в `CmpInOut`

use tokio::sync::mpsc;

use crate::{
    executor::{CmpInOut, ComponentError},
    message::{Message, MsgDataBound, ServiceBound},
};

/// Задача перенаправления сообщений из канала `mpsc` в `CmpInOut`
pub struct MpscToMsgBus<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    /// Входящие сообщения
    pub input: mpsc::Receiver<Message<TMsg>>,

    /// Исходящие сообщения, шина сообщений между компонентами
    pub msg_bus: CmpInOut<TMsg, TService>,
}

impl<TMsg, TService> MpscToMsgBus<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
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
