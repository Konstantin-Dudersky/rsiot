use tokio::sync::mpsc;
use tracing::warn;

use crate::{executor::MsgBusInput, message::MsgDataBound};

use super::{
    BufferBound, COMPONENT_NAME, Error, config::FnInput, internal_message::InternalMessage,
};

pub struct Input<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    pub input: MsgBusInput<TMsg>,
    pub output: mpsc::Sender<InternalMessage<TBuffer>>,
    pub fn_input: FnInput<TMsg, TBuffer>,
}

impl<TMsg, TBuffer> Input<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut buffer = TBuffer::default();

        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            let maybe_buffer = (self.fn_input)(&msg, &buffer);
            let Some(new_buffer) = maybe_buffer else {
                continue;
            };

            buffer = new_buffer.clone();

            let res = self
                .output
                .try_send(InternalMessage::BufferData(new_buffer));

            if res.is_err() {
                warn!("Bus in component {COMPONENT_NAME} is full");
            }
        }

        Err(Error::TaskEndInput)
    }
}
