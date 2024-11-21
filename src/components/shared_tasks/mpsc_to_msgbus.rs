//! Задача перенаправления сообщений из канала `mpsc` в `CmpInOut`

use tokio::sync::mpsc;

use crate::{
    executor::{CmpInOut, ComponentError},
    message::{Message, MsgDataBound},
};

/// Задача перенаправления сообщений из канала `mpsc` в `CmpInOut`
pub struct MpscToMsgBus<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Входящие сообщения
    pub input: mpsc::Receiver<Message<TMsg>>,

    /// Исходящие сообщения, шина сообщений между компонентами
    pub cmp_in_out: CmpInOut<TMsg>,
}

impl<TMsg> MpscToMsgBus<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            self.cmp_in_out.send_output(msg).await?;
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
