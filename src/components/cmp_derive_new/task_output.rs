use tokio::sync::mpsc;

use crate::{
    executor::MsgBusOutput,
    message::{Message, MsgDataBound},
};

use super::{BufferBound, ConfigOutputSend, Error, config::FnOutput, internal_message::InternalMessage};

pub struct Output<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    pub input: mpsc::Receiver<InternalMessage<TBuffer>>,
    pub output: MsgBusOutput<TMsg>,
    pub fn_output: FnOutput<TMsg, TBuffer>,
    pub output_send: ConfigOutputSend,
}

impl<TMsg, TBuffer> Output<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut buffer = TBuffer::default();

        while let Some(int_msg) = self.input.recv().await {
            let msg = match int_msg {
                InternalMessage::BufferData(new_buffer) => match self.output_send {
                    ConfigOutputSend::OnEveryChange => (self.fn_output)(&new_buffer),
                    ConfigOutputSend::Periodic(_) => {
                        buffer = new_buffer;
                        continue;
                    }
                },
                InternalMessage::Period(_) => (self.fn_output)(&buffer),
            };
            let msg = Message::new_custom(msg);
            self.output
                .send(msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }

        Err(Error::TaskEndOutput)
    }
}
