use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};
use tracing::warn;

use crate::{
    components_config::can_general::CanFrame,
    executor::{MsgBusInput, MsgBusLinker},
    message::MsgDataBound,
};

pub struct Input<TMsg, TError, TFnInput>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(&TMsg) -> anyhow::Result<Option<Vec<CanFrame>>>,
{
    pub input: MsgBusInput<TMsg>,
    pub output: mpsc::Sender<CanFrame>,
    pub fn_input: TFnInput,
    pub error_task_end: fn() -> TError,
    pub error_tokio_mpsc_send: fn() -> TError,
}

impl<TMsg, TError, TFnInput> Input<TMsg, TError, TFnInput>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(&TMsg) -> anyhow::Result<Option<Vec<CanFrame>>>,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            let frame = (self.fn_input)(&msg);
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
