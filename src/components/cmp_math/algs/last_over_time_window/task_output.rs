use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::executor::sleep;

use super::{buffer::Buffer, Error, IntMsgBound, Result};

pub struct TaskOutput<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub output: mpsc::Sender<TIntMsg>,
    pub fn_output: fn(f64) -> TIntMsg,
    pub buffer: Arc<Mutex<Buffer>>,
}
impl<TIntMsg> TaskOutput<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub async fn spawn(self) -> Result<()> {
        loop {
            let (last_value, window) = {
                let buffer = self.buffer.lock().await;
                (buffer.last_value, buffer.window)
            };

            let int_msg = (self.fn_output)(last_value);
            self.output.send(int_msg).await.map_err(|_| {
                Error::AlgTaskUnexpectedEnd("AlgLastOverTimeWindow - TaskOutput".into())
            })?;

            sleep(window).await;
        }
    }
}
