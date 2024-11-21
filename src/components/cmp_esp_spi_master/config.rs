use std::time::Duration;

use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::{
    peripheral::Peripheral,
    spi::{Spi, SpiDeviceDriver, SpiDriver},
};

use crate::message::{Message, MsgDataBound};

/// Конфигурация компонента cmp_esp_spi_master
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

    /// Массив конфигураций подчиненных устройств на шине SPI
    pub devices: Vec<ConfigDevice<TMsg>>,

    /// Период вызова функций fn_output всех устройств
    pub fn_output_period: Duration,
}

/// Конфигурация подчиненных устройств на шине SPI
pub struct ConfigDevice<TMsg> {
    /// Пин CS
    pub pin_cs: AnyIOPin,

    /// Функция инициализации
    pub fn_init: for<'a> fn(&mut SpiDeviceDriver<'a, &SpiDriver<'a>>),

    /// Функция преобразования входящих сообщений в команды SPI
    pub fn_input: for<'a> fn(&Message<TMsg>, &mut SpiDeviceDriver<'a, &SpiDriver<'a>>),

    /// Функция преобразования данных из SPI в исходящие сообщения
    pub fn_output: for<'a> fn(&mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> Vec<Message<TMsg>>,
}
