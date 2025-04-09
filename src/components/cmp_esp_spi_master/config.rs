use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::spi::config::{MODE_0, MODE_1, MODE_2, MODE_3};
use esp_idf_svc::hal::{peripheral::Peripheral, spi::Spi};

use crate::components_config::master_device::DeviceTrait;
use crate::components_config::spi_master::{self, ConfigDeviceSpiMode};
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

    /// Пин MISO
    pub pin_miso: AnyIOPin,

    /// Пин MOSI
    pub pin_mosi: AnyIOPin,

    /// Пин SCK
    pub pin_sck: AnyIOPin,

    /// Массив настроек коммуникации с устройствами
    ///
    /// Порядок элементов в этом массиве должен соответствовать порядку устройств в массиве devices
    pub devices_comm_settings: Vec<ConfigDevicesCommSettings>,

    /// Драйвера устройств
    pub devices: Vec<
        Box<dyn DeviceTrait<TMsg, spi_master::FieldbusRequest, spi_master::FieldbusResponse, u8>>,
    >,
}

/// Настройки коммуникации с устройствами
pub struct ConfigDevicesCommSettings {
    /// Пин Chip Select
    pub pin_cs: AnyIOPin,

    /// Частота тактов
    pub baudrate: u32,

    /// Режим работы SPI
    pub spi_mode: ConfigDeviceSpiMode,
}

impl From<ConfigDeviceSpiMode> for esp_idf_svc::hal::spi::config::Mode {
    fn from(value: ConfigDeviceSpiMode) -> Self {
        match value {
            ConfigDeviceSpiMode::Mode0 => MODE_0,
            ConfigDeviceSpiMode::Mode1 => MODE_1,
            ConfigDeviceSpiMode::Mode2 => MODE_2,
            ConfigDeviceSpiMode::Mode3 => MODE_3,
        }
    }
}
