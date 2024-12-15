use tokio::sync::mpsc::Sender;

use crate::{
    components::cmp_esp_spi_master::InnerMessage,
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

pub struct Input<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub input: CmpInOut<TMsg, TService>,
    pub output: Sender<InnerMessage<TMsg>>,
}

impl<TMsg, TService> Input<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.input.recv_input().await {
            self.output.send(InnerMessage::Message(msg)).await.unwrap();
        }
        Ok(())
    }
}
