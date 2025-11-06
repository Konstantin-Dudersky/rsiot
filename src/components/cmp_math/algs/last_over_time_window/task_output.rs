use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    executor::{MsgBusOutput, sleep},
    message::MsgDataBound,
};

use super::{AlgFnOutputMsgbus, AlgOutput, Error, buffer::Buffer};

pub struct TaskOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub output: AlgOutput,
    pub output_msgbus: MsgBusOutput<TMsg>,
    pub fn_output_msgbus: AlgFnOutputMsgbus<TMsg, f64>,
    pub buffer: Arc<Mutex<Buffer>>,
}
impl<TMsg> TaskOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), Error> {
        loop {
            let (last_value, window) = {
                let buffer = self.buffer.lock().await;
                (buffer.last_value, buffer.window)
            };

            let msg = (self.fn_output_msgbus)(&last_value.value);
            if let Some(msg) = msg {
                self.output_msgbus
                    .send(msg.to_message())
                    .await
                    .map_err(|_| Error::SendToMsgbus)?;
            }

            self.output.send(last_value).await.map_err(|_| {
                Error::AlgTaskUnexpectedEnd("AlgLastOverTimeWindow - TaskOutput".into())
            })?;

            sleep(window).await;
        }
    }
}
