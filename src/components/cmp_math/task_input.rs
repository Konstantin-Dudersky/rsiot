use tokio::sync::mpsc;

use crate::{
    executor::MsgBusInput,
    message::{MsgDataBound, ValueTime},
};

use super::Error;

pub struct TaskInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub output: mpsc::Sender<ValueTime>,
    pub fn_input: fn(&TMsg) -> Option<ValueTime>,
}

impl<TMsg> TaskInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            if let Some(vt) = (self.fn_input)(&msg) {
                self.output
                    .send(vt)
                    .await
                    .map_err(|_| Error::TaskInputEnd)?;
            }
        }
        Err(Error::TaskInputEnd)
    }
}
