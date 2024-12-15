use std::{sync::Arc, time::Duration};

use bitvec::prelude::*;
use tokio::{sync::Mutex, time::sleep};

use crate::{
    drivers_i2c::{I2cSlaveAddress, RsiotI2cDriverBase},
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::{state::State, TPinFnOutput};

/// Чтение и обработка входов
pub struct TaskReadInputs<TMsg, TService, Driver>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub in_out: CmpInOut<TMsg, TService>,
    pub driver: Arc<Mutex<Driver>>,
    pub address: I2cSlaveAddress,
    pub pin_and_fn_output: TPinFnOutput<TMsg>,
    pub state: State,
}

impl<TMsg, TService, Driver> TaskReadInputs<TMsg, TService, Driver>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
    Driver: RsiotI2cDriverBase,
{
    pub async fn spawn(&self) -> Result<(), String> {
        let mut status_saved = 0u16;
        let status_saved_bits = status_saved.view_bits_mut::<Lsb0>();
        let mut first_cycle = true;

        loop {
            let state = self.state.to_bytes().await;
            let status_current = {
                let mut driver = self.driver.lock().await;
                driver
                    .write_read(self.address, &state, 2, Duration::from_secs(2))
                    .await
                    .map_err(String::from)?
            };
            let status_current_bits = status_current.view_bits::<Lsb0>();
            for (pin, fn_output) in &self.pin_and_fn_output {
                if (status_current_bits[*pin] != status_saved_bits[*pin]) || first_cycle {
                    let msg = fn_output(!status_current_bits[*pin]);
                    status_saved_bits.set(*pin, status_current_bits[*pin]);
                    let Some(msg) = msg else { continue };
                    self.in_out
                        .send_output(msg)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
            first_cycle = false;

            sleep(Duration::from_millis(100)).await;
        }
    }
}
