use tokio::sync::broadcast::{self, error::RecvError};

use rsiot_messages_core::IMessage;

#[derive(Debug)]
pub struct CmpInput<TMsg>
where
    TMsg: IMessage,
{
    channel: broadcast::Receiver<TMsg>,
}

impl<TMsg> CmpInput<TMsg>
where
    TMsg: IMessage,
{
    pub fn new(channel: broadcast::Receiver<TMsg>) -> Self {
        Self { channel }
    }

    pub async fn recv(&mut self) -> Result<Option<TMsg>, RecvError> {
        let msg = self.channel.recv().await?;
        // TODO добавить проверку на источник сообщения и отбрасывать
        Ok(Some(msg))
    }
}

impl<TMsg> Clone for CmpInput<TMsg>
where
    TMsg: IMessage,
{
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.resubscribe(),
        }
    }
}
