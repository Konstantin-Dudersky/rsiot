use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, i2c::I2c, peripheral::Peripheral};

use crate::{drivers_i2c::I2cDevices, message::MsgDataBound};

/// Конфигурация cmp_esp_i2c_master
pub struct Config<TMsg, TI2c, TPeripheral>
where
    TMsg: MsgDataBound,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
{
    /// Ссылка на аппартный интерфейс I2C
    pub i2c: TI2c,

    /// Пин сигнала SDA
    pub sda: AnyIOPin,

    /// Пин сигнала SCL
    pub scl: AnyIOPin,

    /// Скорость
    pub baudrate: ConfigBaudrate,

    /// true - подключить подтяжку встроенными резисторами
    pub pullup_enable: bool,

    /// Таймаут запроса
    pub timeout: Duration,

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
