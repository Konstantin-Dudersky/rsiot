use std::time::Duration;

use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_svc::hal::{
    peripheral::Peripheral,
    spi::{Spi, SpiDeviceDriver, SpiDriver},
};

use crate::message::{Message, MsgDataBound};

pub struct Config<TMsg, TSpi, TPeripheral>
where
    TMsg: MsgDataBound,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi,
{
    /// Ссылка на аппартный интерфейс SPI
    pub spi: TSpi,

    /// Пин MISO
    pub pin_miso: AnyIOPin,

    /// Пин MOSI
    pub pin_mosi: AnyIOPin,

    /// Пин SCK
    pub pin_sck: AnyIOPin,

    /// Массив устройств на шине SPI
    pub devices: Vec<ConfigDevice<TMsg>>,
}

pub struct ConfigDevice<TMsg> {
    pub pin_cs: AnyIOPin,

    pub fn_init: for<'a> fn(&SpiDeviceDriver<'a, &SpiDriver<'a>>),

    pub fn_input: for<'a> fn(&Message<TMsg>, &SpiDeviceDriver<'a, &SpiDriver<'a>>),

    pub fn_output: fn() -> Vec<Message<TMsg>>,

    pub fn_output_period: Duration,
}
