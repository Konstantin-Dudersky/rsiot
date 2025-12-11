use std::time::Duration;

use crate::{
    components_config::uart_general::{Baudrate, DataBits, Parity, StopBits},
    message::MsgDataBound,
};

/// Конфигурация для компонента cmp_linux_uart_slave
#[derive(Clone)]
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

    /// Время ожидания данных в буфере
    pub timeout: Duration,

    /// # Пример
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub fn_output: fn(&[u8]) -> Result<Option<TMsg>, anyhow::Error>,
}
