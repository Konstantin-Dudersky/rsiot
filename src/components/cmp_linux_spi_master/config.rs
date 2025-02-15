use crate::{
    components_config::{master_device::DeviceTrait, spi_master},
    message::MsgDataBound,
};

/// Конфигурация cmp_linux_spi_master
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Пути к устройствам SPI.
    ///
    /// Пример:
    ///
    /// ```rust
    /// spi_path: vec!["/dev/spidev0.0", "/dev/spidev0.1"]
    /// ```
    pub spi_adapter_path: Vec<&'static str>,

    /// Частота тактов в Гц
    pub baudrate: u32,

    /// Драйвера устройств
    pub devices: Vec<
        Box<dyn DeviceTrait<TMsg, spi_master::FieldbusRequest, spi_master::FieldbusResponse, u8>>,
    >,
}
