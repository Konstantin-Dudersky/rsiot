use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, uart::Uart};

use crate::components_config::master_device::DeviceTrait;
use crate::components_config::uart_general::{
    Baudrate, DataBits, Parity, StopBits, UartRequest, UartResponse,
};
use crate::message::MsgDataBound;

/// Конфигурация cmp_linux_uart_master
pub struct Config<TMsg, TUart, TPeripheral>
where
    TMsg: MsgDataBound,
    TUart: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Uart,
{
    /// Интерфейс uart
    ///
    /// Пример:
    ///
    /// ```rust
    /// // Лучше использовать UART1, поскольку в UART0 могут выводиться логи
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

    /// Пин RTS запроса на передачу
    pub pin_rts: AnyIOPin,

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

    /// TODO - переделать на вектор универсальных устройств
    // pub devices: Vec<TestDevice<TMsg>>,
    pub devices: Vec<Box<dyn DeviceTrait<TMsg, UartRequest, UartResponse, u8>>>,
}
