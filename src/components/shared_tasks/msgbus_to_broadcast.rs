//! Задача перенаправления сообщений из `MsgBusLinker` в  канал `broadcast`

use tokio::{sync::broadcast::Sender, time::error};

use crate::{
    executor::{MsgBusLinker, MsgBusInput},
    message::{Message, MsgDataBound},
};

/// Задача перенаправления сообщений из `MsgBusLinker` в  канал `broadcast`
pub struct MsgBusToBroadcast<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Входящий поток сообщений из входа компонента
    pub msgbus_input: MsgBusInput<TMsg>,

    /// Исходящий поток сообщенийnu
    pub output: Sender<Message<TMsg>>,
}

impl<TMsg> MsgBusToBroadcast<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Ok(msg) = self.msgbus_input.recv().await {
            self.output
                .send(msg)
                .map_err(|e| Error::TokioSyncMpsc(e.to_string()))?;
        }
        Ok(())
    }
}

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    TokioSyncMpsc(String),
}
