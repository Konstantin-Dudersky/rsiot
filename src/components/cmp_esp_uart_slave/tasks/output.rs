use std::time::Duration;

use tokio::time::sleep;

use crate::message::Message;

use super::super::config::TFnOutput;
use super::{Buffer, TaskOutput};

pub struct Output<TMsg, TBufferData> {
    pub output: TaskOutput<Message<TMsg>>,
    pub buffer_data: Buffer<TBufferData>,
    pub fn_output: TFnOutput<TMsg, TBufferData>,
    pub fn_output_period: Duration,
}

impl<TMsg, TBufferData> Output<TMsg, TBufferData> {
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            let msgs = {
                let buffer_data = self.buffer_data.lock().await;
                (self.fn_output)(&buffer_data)
            };
            for msg in msgs {
                self.output
                    .send(msg)
                    .await
                    .map_err(|e| super::Error::TaskOutput(e.to_string()))?;
            }
            sleep(self.fn_output_period).await;
        }
    }
}
