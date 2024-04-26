use std::time::Duration;

use esp_idf_svc::hal::i2c::I2cDriver;

use crate::{
    drivers_i2c::I2cDevices,
    message::{Message, MsgDataBound},
};

/// Конфигурация cmp_esp_i2c_master
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: fn(Message<TMsg>) -> Option<String>,

    /// Таймаут запроса
    pub timeout: Duration,

    pub devices: Vec<I2cDevices>,

    pub i2c_driver: I2cDriver<'static>,
}

/// Скорость шины
#[derive(Clone)]
pub enum ConfigBaudrate {
    /// 100 kHz
    Standard,

    /// 400 kHz
    Fast,
}
