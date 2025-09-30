use crate::{executor::MsgBusLinker, message::MsgDataBound};

use super::{I2cResponse, TaskInput};

pub struct Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: TaskInput<I2cResponse>,
    pub output: MsgBusLinker<TMsg>,
}

impl<TMsg> Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(_response) = self.input.recv().await {}
        Err(super::Error::TaskOutput)
    }
}
