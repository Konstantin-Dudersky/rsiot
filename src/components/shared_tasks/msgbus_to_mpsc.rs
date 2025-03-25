//! Задача перенаправления сообщений из `CmpInOut` в  канал `mpsc`

use tokio::{sync::mpsc::Sender, time::error};

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

/// Задача перенаправления сообщений из `CmpInOut` в  канал `mpsc`
pub struct MsgBusToMpsc<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Входящий поток сообщений из входа компонента
    pub msg_bus: CmpInOut<TMsg>,

    /// Исходящий поток сообщений
    pub output: Sender<Message<TMsg>>,
}

impl<TMsg> MsgBusToMpsc<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Ok(msg) = self.msg_bus.recv_input().await {
            self.output
                .send(msg)
                .await
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
