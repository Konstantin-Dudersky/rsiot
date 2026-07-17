use std::time::Duration;

use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::spi::Spi;
use esp_idf_svc::hal::spi::config::{MODE_0, MODE_1, MODE_2, MODE_3};

use crate::components::shared_tasks::fieldbus_execution::FieldbusDiag;
use crate::components_config::master_device::DeviceTrait;
use crate::components_config::spi_master::{self, ConfigDeviceSpiMode};
use crate::message::MsgDataBound;

// ANCHOR: Config
/// Конфигурация компонента cmp_esp_spi_master
pub struct Config<TMsg, TSpi>
where
    TMsg: MsgDataBound,
    TSpi: Spi + 'static,
{
    /// Ссылка на аппартный интерфейс SPI
    pub spi: TSpi,

    /// Пин MISO
    pub pin_miso: AnyIOPin<'static>,

    /// Пин MOSI
    pub pin_mosi: AnyIOPin<'static>,

    /// Пин SCK
    pub pin_sck: AnyIOPin<'static>,

    /// Массив настроек коммуникации с устройствами
    ///
    /// Порядок элементов в этом массиве должен соответствовать порядку устройств в массиве devices
    pub devices_comm_settings: Vec<ConfigDevicesCommSettings>,

    /// Драйвера устройств
    pub devices:
        Vec<Box<dyn DeviceTrait<TMsg, spi_master::FieldbusRequest, spi_master::FieldbusResponse>>>,

    /// Функция для формирования сообщения диагностики
    pub fn_diag: fn(&FieldbusDiag) -> TMsg,

    /// Период отправки сообщений диагностики
    pub fn_diag_period: Duration,
}
// ANCHOR: Config

// ANCHOR: ConfigDevicesCommSettings
/// Настройки коммуникации с устройствами
pub struct ConfigDevicesCommSettings {
    /// Пин Chip Select
    pub pin_cs: AnyIOPin<'static>,

    /// Частота тактов
    pub baudrate: u32,

    /// Режим работы SPI
    pub spi_mode: ConfigDeviceSpiMode,
}
// ANCHOR: ConfigDevicesCommSettings

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
