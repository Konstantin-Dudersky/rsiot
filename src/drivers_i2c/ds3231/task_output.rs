use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::sleep};

use crate::{
    drivers_i2c::{I2cSlaveAddress, RsiotI2cDriverBase},
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::data_models;

pub struct OutputData {
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
/// Задача чтения данных с модуля
pub struct TaskOutput<TMsg, Driver>
where
    Driver: RsiotI2cDriverBase,
    TMsg: MsgDataBound,
{
    pub address: I2cSlaveAddress,
    pub period: Duration,
    pub driver: Arc<Mutex<Driver>>,
    pub fn_output: fn(OutputData) -> Option<Vec<Message<TMsg>>>,
    pub in_out: CmpInOut<TMsg>,
}

impl<TMsg, Driver> TaskOutput<TMsg, Driver>
where
    Driver: RsiotI2cDriverBase,
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), String> {
        loop {
            sleep(self.period).await;

            let mut driver = self.driver.lock().await;
            let res = driver.read(self.address, 19).await?;

            let second = data_models::Second::new_from_device(res[0]);
            let minute = data_models::Minute::new_from_device(res[1]);
            let hour = data_models::Hour::new_from_device(res[2]);
            let day = data_models::Day::new_from_device(res[4]);
            let month = data_models::Month::new_from_device(res[5]);
            let year = data_models::Year::new_from_device(res[6]);

            let output = OutputData {
                year: year.get(),
                month: month.get(),
                day: day.get(),
                hour: hour.get(),
                minute: minute.get(),
                second: second.get(),
            };

            let msgs = (self.fn_output)(output);
            let Some(msgs) = msgs else { continue };
            for msg in msgs {
                self.in_out
                    .send_output(msg)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
    }
}
