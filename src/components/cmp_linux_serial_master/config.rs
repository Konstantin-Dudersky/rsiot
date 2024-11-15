use std::time::Duration;

use crate::components_config::uart_general::*;
use crate::message::{Message, MsgDataBound};

use super::devices::TestDevice;

/// Конфигурация cmp_linux_uart
#[derive(Clone, Debug)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Название порта
    pub port: &'static str,

    /// Скорость сетевого обмена
    pub baudrate: Baudrate,

    /// Кол-во бит данных
    pub data_bits: DataBits,

    /// Бит четности
    pub parity: Parity,

    /// Кол-во стоповых бит
    pub stop_bits: StopBits,

    /// Задержка перед чтением буфера после записи
    ///
    /// Чем выше скорость, тем меньшую задержку можно ставить
    ///
    /// Чем больше размер посылки, тем большую задержку нужно ставить
    pub delay_between_write_and_read: Duration,

    /// TODO - переделать на вектор универсальных устройств
    pub devices: Vec<TestDevice<TMsg>>,

    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: fn(Message<TMsg>) -> Option<String>,

    /// # Пример
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub fn_output: fn(String) -> Vec<Message<TMsg>>,
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
            delay_between_write_and_read: Duration::from_millis(100),
            fn_input: |_| None,
            fn_output: |_| vec![],
            devices: vec![],
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
