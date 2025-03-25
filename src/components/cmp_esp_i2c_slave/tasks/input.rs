use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::super::FnInput;

pub struct Input<TMsg, TBufferData>
where
    TMsg: MsgDataBound,
{
    pub msg_bus: CmpInOut<TMsg>,
    pub fn_input: FnInput<TMsg, TBufferData>,
    pub buffer_data: Arc<Mutex<TBufferData>>,
}

impl<TMsg, TBufferData> Input<TMsg, TBufferData>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.msg_bus.recv_input().await {
            let mut buffer_data = self.buffer_data.lock().await;
            (self.fn_input)(&msg, &mut buffer_data);
        }
        Ok(())
    }
}
