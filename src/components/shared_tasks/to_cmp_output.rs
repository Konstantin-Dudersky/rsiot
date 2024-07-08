use tokio::sync::mpsc;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

/// Задача перенаправления сообещений из канала `mpsc` в `CmpInOut`
pub struct ToCmpOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: mpsc::Receiver<Message<TMsg>>,
    pub cmp_in_out: CmpInOut<TMsg>,
}

impl<TMsg> ToCmpOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            self.cmp_in_out.send_output(msg).await.unwrap();
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {}
