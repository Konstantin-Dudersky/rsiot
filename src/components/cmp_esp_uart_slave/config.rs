use esp_idf_hal::{uart, units::Hertz};
use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, uart::Uart};

use crate::message::{Message, MsgDataBound};

pub use crate::components_config::uart_general::*;

/// Функция для работы коммуникации I2C
pub type TFnUartComm<TRequest, TResponse, TBufferData> =
    fn(TRequest, &mut TBufferData) -> anyhow::Result<TResponse>;

/// Конфигурация cmp_esp_uart_slave
pub struct Config<TMsg, TUart, TPeripheral, TRequest, TResponse, TBufferData>
where
    TMsg: MsgDataBound,
    TUart: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Uart,
    TRequest: RequestResponseBound,
    TResponse: RequestResponseBound,
{
    /// Адрес устройства на шине
    pub address: u8,

    /// Интерфейс uart
    ///
    /// Пример:
    ///
    /// ```rust
    /// uart: peripherals.uart1
    /// ```
    pub uart: TUart,

    /// Пин RX
    ///
    /// Пример:
    ///
    /// ```rust
    /// pin_rx: peripherals.pins.gpio20.into(),
    /// ```
    pub pin_rx: AnyIOPin,

    /// Пин TX
    ///
    /// Пример:
    ///
    /// ```rust
    /// pin_tx: peripherals.pins.gpio21.into(),
    /// ```
    pub pin_tx: AnyIOPin,

    /// Скорость сетевого обмена
    pub baudrate: Baudrate,

    /// Кол-во бит данных
    pub data_bits: DataBits,

    /// Бит четности
    pub parity: Parity,

    /// Кол-во стоповых бит
    pub stop_bits: StopBits,

    /// Структура для хранения буферных данных
    pub buffer_data_default: TBufferData,

    /// Функция коммуникации по UART
    pub fn_uart_comm: TFnUartComm<TRequest, TResponse, TBufferData>,

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

impl From<Baudrate> for Hertz {
    fn from(value: Baudrate) -> Self {
        Hertz(value.into())
    }
}

impl From<DataBits> for uart::config::DataBits {
    fn from(value: DataBits) -> Self {
        match value {
            DataBits::_5 => Self::DataBits5,
            DataBits::_6 => Self::DataBits6,
            DataBits::_7 => Self::DataBits7,
            DataBits::_8 => Self::DataBits8,
        }
    }
}

impl From<Parity> for uart::config::Parity {
    fn from(value: Parity) -> Self {
        match value {
            Parity::None => Self::ParityNone,
            Parity::Even => Self::ParityEven,
            Parity::Odd => Self::ParityOdd,
        }
    }
}

impl From<StopBits> for uart::config::StopBits {
    fn from(value: StopBits) -> Self {
        match value {
            StopBits::_1 => Self::STOP1,
            StopBits::_1p5 => Self::STOP1P5,
            StopBits::_2 => Self::STOP2,
        }
    }
}
