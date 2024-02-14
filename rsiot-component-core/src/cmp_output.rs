use tokio::sync::mpsc::{self, error::SendError};

use rsiot_messages_core::{msg_meta::ServiceId, IMessage};

#[derive(Clone)]
pub struct CmpOutput<TMsg>
where
    TMsg: IMessage,
{
    channel: mpsc::Sender<TMsg>,
    service_id: ServiceId,
}

impl<TMsg> CmpOutput<TMsg>
where
    TMsg: IMessage,
{
    pub fn new(channel: mpsc::Sender<TMsg>, service_id: ServiceId) -> Self {
        Self {
            channel,
            service_id,
        }
    }

    pub async fn send(&self, msg: TMsg) -> Result<(), SendError<TMsg>> {
        self.channel.send(msg).await
    }
}
