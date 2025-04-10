use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, uart::Uart};

use crate::message::{Message, MsgDataBound};

use crate::components_config::uart_general::*;

/// Функция преобразования входных сообщений в данные для передачи I2C
pub type TFnInput<TMsg, TBufferData> = fn(&Message<TMsg>, &mut TBufferData);

/// Функция для преобразования полученных данных I2C в исходящие сообщения
pub type TFnOutput<TMsg, TBufferData> = fn(&TBufferData) -> Vec<Message<TMsg>>;

/// Функция для работы коммуникации I2C
pub type TFnUartComm<TRequest, TResponse, TBufferData> =
    fn(TRequest, &mut TBufferData) -> anyhow::Result<TResponse>;

/// Конфигурация cmp_esp_uart_slave
pub struct Config<TMsg, TUart, TPeripheral, TBufferData>
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

    /// Структура для хранения буферных данных
    pub buffer_data_default: TBufferData,

    /// Функция преобразования входных сообщений в данные для передачи по UART
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: TFnInput<TMsg, TBufferData>,

    /// Функция коммуникации по UART
    pub fn_uart_comm: TFnUartComm<FieldbusRequest, FieldbusResponse, TBufferData>,

    /// Функция для преобразования полученных данных UART в исходящие сообщения.
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub fn_output: TFnOutput<TMsg, TBufferData>,

    /// Периодичность генерирования исходящих сообщений
    pub fn_output_period: Duration,
}
