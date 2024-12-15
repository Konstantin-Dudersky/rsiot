use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::{super::config::TFnInput, Buffer};

pub struct Input<TMsg, TService, TBufferData>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub buffer_data: Buffer<TBufferData>,
    pub msg_bus: CmpInOut<TMsg, TService>,
    pub fn_input: TFnInput<TMsg, TBufferData>,
}

impl<TMsg, TService, TBufferData> Input<TMsg, TService, TBufferData>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.msg_bus.recv_input().await {
            let mut buffer_data = self.buffer_data.lock().await;
            (self.fn_input)(&msg, &mut buffer_data);
        }
        Ok(())
    }
}
