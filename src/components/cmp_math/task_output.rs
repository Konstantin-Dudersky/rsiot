use tokio::sync::mpsc;

use crate::{
    executor::MsgBusOutput,
    message::{MsgDataBound, ValueTime},
};

use super::Error;

pub struct TaskOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: mpsc::Receiver<ValueTime>,
    pub output: MsgBusOutput<TMsg>,
    pub fn_output: fn(&ValueTime) -> Option<TMsg>,
}

impl<TMsg> TaskOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(vt) = self.input.recv().await {
            let msg = (self.fn_output)(&vt);
            let Some(msg) = msg else { continue };
            self.output
                .send(msg.to_message())
                .await
                .map_err(|_| Error::TaskOutputEnd)?;
        }
        Err(Error::TaskOutputEnd)
    }
}
