use std::sync::Arc;
use std::time::Duration;

use tokio::{sync::Mutex, time::sleep};

use crate::{
    drivers_i2c::{I2cSlaveAddress, RsiotI2cDriverBase},
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

pub struct Output<TMsg, TDriver>
where
    TMsg: MsgDataBound,
    TDriver: RsiotI2cDriverBase,
{
    pub address: I2cSlaveAddress,
    pub driver: Arc<Mutex<TDriver>>,
    pub fn_output: fn(Vec<u8>) -> Vec<Message<TMsg>>,
    pub msg_bus: CmpInOut<TMsg>,
    pub period: Duration,
}

impl<TMsg, TDriver> Output<TMsg, TDriver>
where
    TMsg: MsgDataBound,
    TDriver: RsiotI2cDriverBase,
{
    pub async fn spawn(self) -> Result<(), String> {
        loop {
            let mut driver = self.driver.lock().await;
            driver.read(self.address, 2).await.unwrap();
            sleep(self.period).await
        }
    }
}
