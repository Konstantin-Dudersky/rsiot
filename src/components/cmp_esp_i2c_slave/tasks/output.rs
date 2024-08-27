use std::sync::Arc;

use esp_idf_hal::i2c::I2cSlaveDriver;
use tokio::sync::Mutex;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::super::FnOutput;

pub struct Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msg_bus: CmpInOut<TMsg>,
    pub driver: Arc<Mutex<I2cSlaveDriver<'static>>>,
    pub fn_output: FnOutput<TMsg>,
}

impl<TMsg> Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        Ok(())
    }
}
