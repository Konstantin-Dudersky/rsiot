use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

use crate::message::{Message, MsgDataBound};

use super::super::FnOutput;

pub struct Output<TMsg, TBufferData>
where
    TMsg: MsgDataBound,
{
    pub output: mpsc::Sender<Message<TMsg>>,
    pub fn_output: FnOutput<TMsg, TBufferData>,
    pub fn_output_period: Duration,
    pub buffer_data: Arc<Mutex<TBufferData>>,
}

impl<TMsg, TBufferData> Output<TMsg, TBufferData>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            let msgs;
            {
                let buffer_data = self.buffer_data.lock().await;
                msgs = (self.fn_output)(&buffer_data);
            }
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
