use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    drivers_i2c::{I2cSlaveAddress, RsiotI2cDriverBase},
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::data_models;

/// Структура входных данных для задания времени
pub struct InputData {
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

pub struct TaskInput<TMsg, Driver>
where
    Driver: RsiotI2cDriverBase,
    TMsg: MsgDataBound,
{
    pub address: I2cSlaveAddress,
    pub driver: Arc<Mutex<Driver>>,
    pub fn_input: fn(Message<TMsg>) -> Option<InputData>,
    pub in_out: CmpInOut<TMsg>,
}

impl<TMsg, Driver> TaskInput<TMsg, Driver>
where
    Driver: RsiotI2cDriverBase,
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), String> {
        while let Ok(msg) = self.in_out.recv_input().await {
            let input_data = (self.fn_input)(msg);
            let Some(input_data) = input_data else {
                continue;
            };

            let mut driver = self.driver.lock().await;

            let request: Vec<u8> = vec![
                0x00,
                data_models::Second::new_from_dec(input_data.second).get_bcd(),
                data_models::Minute::new_from_dec(input_data.minute).get_bcd(),
                data_models::Hour::new_from_dec(input_data.hour).get_bcd(),
                0x01, // день ненели
                data_models::Day::new_from_dec(input_data.day).get_bcd(),
                data_models::Month::new_from_dec(input_data.month).get_bcd(),
                data_models::Year::new_from_dec(input_data.year).get_bcd(),
            ];
            driver.write(self.address, &request).await.unwrap();
        }
        Ok(())
    }
}
