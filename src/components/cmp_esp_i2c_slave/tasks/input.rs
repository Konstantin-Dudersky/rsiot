use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::super::FnInput;

pub struct Input<TMsg, TBufferData, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub msg_bus: CmpInOut<TMsg, TService>,
    pub fn_input: FnInput<TMsg, TBufferData>,
    pub buffer_data: Arc<Mutex<TBufferData>>,
}

impl<TMsg, TBufferData, TService> Input<TMsg, TBufferData, TService>
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
