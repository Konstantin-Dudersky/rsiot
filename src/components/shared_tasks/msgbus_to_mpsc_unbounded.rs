//! Задача перенаправления сообщений из `CmpInOut` в  канал `mpsc`

use tokio::{sync::mpsc::UnboundedSender, time::error};

use crate::{
    executor::{CmpInOut, MsgBusInput},
    message::{Message, MsgDataBound},
};

/// Задача перенаправления сообщений из `CmpInOut` в  канал `mpsc`
pub struct Task<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Входящий поток сообщений из входа компонента
    pub msgbus_input: MsgBusInput<TMsg>,

    /// Исходящий поток сообщений
    pub output: UnboundedSender<Message<TMsg>>,
}

impl<TMsg> Task<TMsg>
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
