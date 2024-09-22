use std::sync::Arc;

use tokio::sync::Mutex;

use crate::message::Message;

use super::{
    super::config::{Buffer, FnInput},
    I2cRequest, TaskInput, TaskOutput,
};

pub struct Input<TMsg> {
    pub input: TaskInput<Message<TMsg>>,
    pub output: TaskOutput<I2cRequest>,
    pub fn_input: FnInput<TMsg>,
    pub buffer: Arc<Mutex<Buffer>>,
}

impl<TMsg> Input<TMsg> {
    pub async fn spawn(mut self) -> super::Result<()> {
        let mut old_buffer = { self.buffer.lock().await.clone() };

        while let Some(msg) = self.input.recv().await {
            let buffer = {
                let mut buffer = self.buffer.lock().await;
                (self.fn_input)(&msg, &mut buffer);
                if *buffer == old_buffer {
                    continue;
                } else {
                    old_buffer = buffer.clone();
                    old_buffer.clone()
                }
            };
            let request = I2cRequest::SetOutputs(buffer.into());
            self.output
                .send(request)
                .await
                .map_err(|_| super::Error::TokioTaskSend)?;
        }
        Err(super::Error::TaskInput)
    }
}
