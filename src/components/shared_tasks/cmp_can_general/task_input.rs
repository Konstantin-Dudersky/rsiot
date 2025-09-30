use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};
use tracing::warn;

use crate::{
    components_config::can_general::{BufferBound, CanFrame},
    executor::{MsgBusInput, MsgBusLinker},
    message::MsgDataBound,
};

pub struct Input<TMsg, TError, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    pub input: MsgBusInput<TMsg>,
    pub output: mpsc::Sender<CanFrame>,
    pub buffer: Arc<Mutex<TBuffer>>,
    pub fn_input: fn(&TMsg, &mut TBuffer) -> anyhow::Result<Option<Vec<CanFrame>>>,
    pub error_task_end: fn() -> TError,
    pub error_tokio_mpsc_send: fn() -> TError,
}

impl<TMsg, TError, TBuffer> Input<TMsg, TError, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            let frame = {
                let mut buffer = self.buffer.lock().await;
                (self.fn_input)(&msg, &mut buffer)
            };
            let frame = match frame {
                Ok(v) => v,
                Err(e) => {
                    warn!("Error in fn_input of CAN Input task: {}", e);
                    continue;
                }
            };
            let Some(frames) = frame else {
                continue;
            };

            for frame in frames {
                self.output
                    .send(frame)
                    .await
                    .map_err(|_| (self.error_tokio_mpsc_send)())?;
            }
        }

        Err((self.error_task_end)())
    }
}
