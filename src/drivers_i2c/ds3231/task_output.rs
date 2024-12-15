use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::sleep};
use tracing::debug;

use crate::{
    drivers_i2c::{I2cSlaveAddress, RsiotI2cDriverBase},
    executor::CmpInOut,
    message::{Message, MsgDataBound, ServiceBound},
};

use super::data_models;

/// Структура выходных данных
pub struct OutputData {
    /// Год
    pub year: u8,
    /// Месяц
    pub month: u8,
    /// День
    pub day: u8,
    /// Час
    pub hour: u8,
    /// Минуты
    pub minute: u8,
    /// Секунды
    pub second: u8,
}
/// Задача чтения данных с модуля
pub struct TaskOutput<TMsg, TService, Driver>
where
    Driver: RsiotI2cDriverBase,
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub address: I2cSlaveAddress,
    pub period: Duration,
    pub driver: Arc<Mutex<Driver>>,
    pub fn_output: fn(OutputData) -> Option<Vec<Message<TMsg>>>,
    pub in_out: CmpInOut<TMsg, TService>,
}

impl<TMsg, TService, Driver> TaskOutput<TMsg, TService, Driver>
where
    Driver: RsiotI2cDriverBase,
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub async fn spawn(self) -> Result<(), String> {
        loop {
            let mut driver = self.driver.lock().await;
            let res = driver
                .write_read(self.address, &[0x00], 19, Duration::from_secs(2))
                .await?;

            let second = data_models::Second::new_from_bcd(res[0]);
            let minute = data_models::Minute::new_from_bcd(res[1]);
            let hour = data_models::Hour::new_from_bcd(res[2]);
            let day = data_models::Day::new_from_bcd(res[4]);
            let month = data_models::Month::new_from_bcd(res[5]);
            let year = data_models::Year::new_from_bcd(res[6]);

            let res = driver
                .write_read(self.address, &[0x0E], 2, Duration::from_secs(2))
                .await?;
            debug!("Control: {:?}", res[0]);
            debug!("Status: {:?}", res[1]);

            let output = OutputData {
                year: year.get_dec(),
                month: month.get_dec(),
                day: day.get_dec(),
                hour: hour.get_dec(),
                minute: minute.get_dec(),
                second: second.get_dec(),
            };

            let msgs = (self.fn_output)(output);
            let Some(msgs) = msgs else { continue };
            for msg in msgs {
                self.in_out
                    .send_output(msg)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            sleep(self.period).await;
        }
    }
}
