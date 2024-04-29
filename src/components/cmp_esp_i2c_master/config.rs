use std::time::Duration;

use esp_idf_svc::hal::i2c::I2cDriver;

use crate::{drivers_i2c::I2cDevices, message::MsgDataBound};

/// Конфигурация cmp_esp_i2c_master
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Таймаут запроса
    pub timeout: Duration,

    /// Ссылка на аппаратный драйвер шины I2C контроллера ESP
    pub i2c_driver: I2cDriver<'static>,

    /// Конфигурация устройств по шине
    pub devices: Vec<I2cDevices<TMsg>>,
}

/// Скорость шины
#[derive(Clone)]
pub enum ConfigBaudrate {
    /// 100 kHz
    Standard,

    /// 400 kHz
    Fast,
}
