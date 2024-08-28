use std::sync::Arc;

use esp_idf_hal::i2c::I2cSlaveDriver;
use tokio::sync::Mutex;
use tracing::info;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::super::FnInput;

pub struct Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msg_bus: CmpInOut<TMsg>,
    pub driver: Arc<Mutex<I2cSlaveDriver<'static>>>,
    pub fn_input: FnInput<TMsg>,
}

impl<TMsg> Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.msg_bus.recv_input().await {
            let buffer = (self.fn_input)(&msg);
            let Some(buffer) = buffer else { continue };
            let mut driver = self.driver.lock().await;
            info!("Send to I2C: {:?}", buffer);
            // driver.write(&buffer, 1000).unwrap();
        }

        Ok(())
    }
}
