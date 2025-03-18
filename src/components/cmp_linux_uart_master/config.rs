use std::time::Duration;

use linux_embedded_hal::serialport;

use crate::components_config::master_device::DeviceTrait;
use crate::components_config::uart_general::{
    Baudrate, DataBits, Parity, StopBits, UartRequest, UartResponse,
};
use crate::message::MsgDataBound;

/// Конфигурация cmp_linux_uart_master
#[derive(Debug)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Название порта
    ///
    /// Примеры:
    ///
    /// ```
    /// port: "/dev/ttyUSB0"
    /// port: "/dev/ttyAMA0"
    /// ```
    pub port: &'static str,

    /// Скорость сетевого обмена
    pub baudrate: Baudrate,

    /// Кол-во бит данных
    pub data_bits: DataBits,

    /// Бит четности
    pub parity: Parity,

    /// Кол-во стоповых бит
    pub stop_bits: StopBits,

    /// Время ожидания ответа
    pub timeout: Duration,

    /// Название чипа gpio в системе.
    ///
    /// Примеры:
    ///
    /// ```rust
    /// gpio_chip: "/dev/gpiochip0"
    /// ```
    pub gpio_chip: &'static str,

    /// Номер пина для сигнала RTS (ready to send).
    ///
    /// Примеры:
    ///
    /// ```rust
    /// // На raspberry pi 17 пин - 11 физ. вывод на гребенке
    /// pin_rts: Some(17),
    ///
    /// // Если пин RTS не нужен
    /// pin_rts: None
    /// ```
    pub pin_rts: Option<u32>,

    /// TODO - переделать на вектор универсальных устройств
    // pub devices: Vec<TestDevice<TMsg>>,
    pub devices: Vec<Box<dyn DeviceTrait<TMsg, UartRequest, UartResponse, u8>>>,
}

impl<TMsg> Default for Config<TMsg>
where
    TMsg: MsgDataBound,
{
    fn default() -> Self {
        Self {
            port: "/dev/ttyAMA0",
            baudrate: Baudrate::default(),
            data_bits: DataBits::default(),
            parity: Parity::default(),
            stop_bits: StopBits::default(),
            devices: vec![],
            timeout: Duration::from_millis(100),
            gpio_chip: "/dev/gpiochip0",
            pin_rts: Some(17),
        }
    }
}

impl From<DataBits> for serialport::DataBits {
    fn from(value: DataBits) -> Self {
        match value {
            DataBits::_5 => serialport::DataBits::Five,
            DataBits::_6 => serialport::DataBits::Six,
            DataBits::_7 => serialport::DataBits::Seven,
            DataBits::_8 => serialport::DataBits::Eight,
        }
    }
}

impl From<Parity> for serialport::Parity {
    fn from(value: Parity) -> Self {
        match value {
            Parity::None => serialport::Parity::None,
            Parity::Even => serialport::Parity::Even,
            Parity::Odd => serialport::Parity::Odd,
        }
    }
}

impl From<StopBits> for serialport::StopBits {
    fn from(value: StopBits) -> Self {
        match value {
            StopBits::_1 => serialport::StopBits::One,
            StopBits::_1p5 => unimplemented!("Stop bit 1.5 not implemented"),
            StopBits::_2 => serialport::StopBits::Two,
        }
    }
}
