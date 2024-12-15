use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::{I2cResponse, TaskInput};

pub struct Output<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub input: TaskInput<I2cResponse>,
    pub output: CmpInOut<TMsg, TService>,
}

impl<TMsg, TService> Output<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(_response) = self.input.recv().await {}
        Err(super::Error::TaskOutput)
    }
}
