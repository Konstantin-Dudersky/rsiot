use tokio::sync::mpsc::Sender;

use crate::{
    components::cmp_esp_spi_master::InnerMessage, executor::CmpInOut, message::MsgDataBound,
};

pub struct Input<TMsg> {
    pub input: CmpInOut<TMsg>,
    pub output: Sender<InnerMessage<TMsg>>,
}

impl<TMsg> Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) {
        while let Ok(msg) = self.input.recv_input().await {
            self.output.send(InnerMessage::Message(msg)).await.unwrap();
        }
    }
}
