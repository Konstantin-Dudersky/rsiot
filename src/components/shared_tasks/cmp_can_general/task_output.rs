use tokio::sync::mpsc;

use crate::{
    components_config::can_general::CanFrame,
    executor::{MsgBusLinker, MsgBusOutput},
    message::{Message, MsgDataBound},
};

pub struct Output<TMsg, TError>
where
    TMsg: MsgDataBound,
{
    pub input: mpsc::Receiver<CanFrame>,
    pub output: MsgBusOutput<TMsg>,
    pub fn_output: fn(CanFrame) -> Option<Vec<TMsg>>,
    pub error_task_end: fn() -> TError,
    pub error_tokio_mpsc_send: fn() -> TError,
}

impl<TMsg, TError> Output<TMsg, TError>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        while let Some(frame) = self.input.recv().await {
            let msgs = (self.fn_output)(frame);
            let Some(msgs) = msgs else { continue };

            for msg in msgs {
                let msg = Message::new_custom(msg);
                self.output
                    .send(msg)
                    .await
                    .map_err(|_| (self.error_tokio_mpsc_send)())?;
            }
        }

        Err((self.error_task_end)())
    }
}
