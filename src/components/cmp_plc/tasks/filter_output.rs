use tokio::sync::mpsc;

use crate::{
    components::shared_tasks::filter_identical_data,
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

pub struct FilterOutputMsgs<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input_msg_channel: mpsc::Receiver<Message<TMsg>>,
    pub cmp_output: CmpInOut<TMsg>,
}

impl<TMsg> FilterOutputMsgs<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        filter_identical_data(self.input_msg_channel, output);

        // while let Some(msg) = self.input_msg_channel.recv().await {
        //     self.cmp_output.send_output(msg).await.unwrap();
        // }

        // Ok(())
    }
}
