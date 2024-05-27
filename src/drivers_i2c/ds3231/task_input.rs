use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    drivers_i2c::{I2cSlaveAddress, RsiotI2cDriverBase},
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

pub struct InputData {
    pub second: u8,
}

pub struct TaskInput<TMsg, Driver>
where
    Driver: RsiotI2cDriverBase,
    TMsg: MsgDataBound,
{
    pub driver: Arc<Mutex<Driver>>,
    pub fn_input: fn(Message<TMsg>) -> Option<InputData>,
    pub in_out: CmpInOut<TMsg>,
}

impl<TMsg, Driver> TaskInput<TMsg, Driver>
where
    Driver: RsiotI2cDriverBase,
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) {
        while let Ok(msg) = self.in_out.recv_input().await {
            let input_data = (self.fn_input)(msg);
            let Some(input_data) = input_data else {
                continue;
            };
        }
    }
}
