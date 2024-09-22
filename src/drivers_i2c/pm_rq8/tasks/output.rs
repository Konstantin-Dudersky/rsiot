use crate::executor::CmpInOut;

use super::{I2cResponse, TaskInput};

pub struct Output<TMsg> {
    pub input: TaskInput<I2cResponse>,
    pub output: CmpInOut<TMsg>,
}

impl<TMsg> Output<TMsg> {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(_response) = self.input.recv().await {}
        Err(super::Error::TaskOutput)
    }
}
