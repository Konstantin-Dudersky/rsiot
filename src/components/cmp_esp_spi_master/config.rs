use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::{peripheral::Peripheral, spi::Spi};

use crate::components_config::master_device::DeviceTrait;
use crate::components_config::spi_master;
use crate::message::MsgDataBound;

/// Конфигурация компонента cmp_esp_spi_master
pub struct Config<TMsg, TSpi, TPeripheral>
where
    TMsg: MsgDataBound,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi,
{
    /// Ссылка на аппартный интерфейс SPI
    pub spi: TSpi,

    /// Частота тактов
    pub baudrate: u32,

    /// Пин MISO
    pub pin_miso: AnyIOPin,

    /// Пин MOSI
    pub pin_mosi: AnyIOPin,

    /// Пин SCK
    pub pin_sck: AnyIOPin,

    /// Массив пинов CS
    pub pin_cs: Vec<AnyIOPin>,

    /// Драйвера устройств
    pub devices: Vec<
        Box<dyn DeviceTrait<TMsg, spi_master::FieldbusRequest, spi_master::FieldbusResponse, u8>>,
    >,
}
