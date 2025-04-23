use std::{sync::Arc, time::Duration};

use tokio::sync::{broadcast, Mutex};

use super::{buffer::Buffer, Error, IntMsgBound, Result};

pub struct TaskInput<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub input: broadcast::Receiver<TIntMsg>,
    pub fn_input_value: fn(TIntMsg) -> Option<f64>,
    pub fn_input_window: fn(TIntMsg) -> Option<Duration>,
    pub buffer: Arc<Mutex<Buffer>>,
}
impl<TIntMsg> TaskInput<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Ok(msg) = self.input.recv().await {
            let value = (self.fn_input_value)(msg);
            if let Some(value) = value {
                let mut buffer = self.buffer.lock().await;
                buffer.last_value = value;
            };

            let value = (self.fn_input_window)(msg);
            if let Some(window) = value {
                let mut buffer = self.buffer.lock().await;
                buffer.window = window;
            };
        }

        let err = String::from("AlgLastOverTimeWindow - TaskInput");
        Err(Error::AlgTaskUnexpectedEnd(err))
    }
}
