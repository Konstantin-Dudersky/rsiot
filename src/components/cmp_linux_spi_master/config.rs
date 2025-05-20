use linux_embedded_hal::spidev::SpiModeFlags;

use crate::{
    components_config::{
        master_device::DeviceTrait,
        spi_master::{self, ConfigDeviceSpiMode},
    },
    message::MsgDataBound,
};

/// Конфигурация cmp_linux_spi_master
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Массив настроек коммуникации с устройствами
    ///
    /// Порядок элементов в этом массиве должен соответствовать порядку устройств в массиве devices
    pub devices_comm_settings: Vec<ConfigDevicesCommSettings>,

    /// Драйвера устройств
    pub devices:
        Vec<Box<dyn DeviceTrait<TMsg, spi_master::FieldbusRequest, spi_master::FieldbusResponse>>>,
}

/// Настройки коммуникации с устройствами
pub struct ConfigDevicesCommSettings {
    /// Конфигурация устройства Linux
    pub linux_device: LinuxDevice,

    /// Частота тактов
    pub baudrate: u32,

    /// Режим работы SPI
    pub spi_mode: ConfigDeviceSpiMode,
}

/// Конфигурация устройства Linux
pub enum LinuxDevice {
    /// Только SPI
    Spi {
        /// Устройство SPI, например "/dev/spidev0.0"
        dev_spi: String,
    },
    /// SPI + chip select, который управляется через GPIO
    SpiWithCs {
        /// Устройство SPI, например "/dev/spidev0.0"
        dev_spi: String,
        /// Устройство SPI, например  "/dev/gpiochip0"
        dev_gpio: String,
        /// Номер линии GPIO. 0 .. 31
        gpio_line: u8,
    },
}

impl From<ConfigDeviceSpiMode> for SpiModeFlags {
    fn from(value: ConfigDeviceSpiMode) -> Self {
        match value {
            ConfigDeviceSpiMode::Mode0 => SpiModeFlags::SPI_MODE_0,
            ConfigDeviceSpiMode::Mode1 => SpiModeFlags::SPI_MODE_1,
            ConfigDeviceSpiMode::Mode2 => SpiModeFlags::SPI_MODE_2,
            ConfigDeviceSpiMode::Mode3 => SpiModeFlags::SPI_MODE_3,
        }
    }
}
