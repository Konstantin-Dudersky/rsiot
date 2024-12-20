use std::{sync::Arc, time::Duration};

use tokio::sync::Mutex;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound, ServiceBound},
};

use super::{
    super::{I2cSlaveAddress, RsiotI2cDriverBase},
    state::State,
};

/// Обработка и запись выходов
pub struct TaskWriteOutput<TMsg, TService, Driver>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
    Driver: RsiotI2cDriverBase,
{
    pub in_out: CmpInOut<TMsg, TService>,
    pub fn_input: fn(Message<TMsg>) -> Option<bool>,
    pub state: State,
    pub driver: Arc<Mutex<Driver>>,
    pub address: I2cSlaveAddress,
    pub pin: usize,
}

impl<TMsg, TService, Driver> TaskWriteOutput<TMsg, TService, Driver>
where
    Driver: RsiotI2cDriverBase,
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub async fn spawn(&mut self) -> Result<(), String> {
        while let Ok(msg) = self.in_out.recv_input().await {
            let value = (self.fn_input)(msg);
            let Some(value) = value else { continue };
            let state_bytes = {
                match value {
                    true => self.state.set_output_high(self.pin).await,
                    false => self.state.set_output_low(self.pin).await,
                }
                self.state.to_bytes().await
            };
            {
                let mut driver = self.driver.lock().await;
                driver
                    .write_read(self.address, &state_bytes, 2, Duration::from_secs(2))
                    .await?;
            }
        }
        Ok(())
    }
}
