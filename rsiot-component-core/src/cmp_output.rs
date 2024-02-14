use tokio::sync::mpsc::{self, error::SendError};

use rsiot_messages_core::IMessage;

#[derive(Clone, Debug)]
pub struct CmpOutput<TMsg>
where
    TMsg: IMessage,
{
    channel: mpsc::Sender<TMsg>,
}

impl<TMsg> CmpOutput<TMsg>
where
    TMsg: IMessage,
{
    pub fn new(channel: mpsc::Sender<TMsg>) -> Self {
        Self { channel }
    }

    pub async fn send(&self, msg: TMsg) -> Result<(), SendError<TMsg>> {
        self.channel.send(msg).await
    }
}
