use std::time::Duration;

use crate::message::{Message, MsgDataBound};

/// Конфигурация cmp_esp_i2c_master
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
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

    /// Скорость шины
    pub baudrate: ConfigBaudrate,

    /// Таймаут запроса
    pub timeout: Duration,
}

/// Скорость шины
#[derive(Clone)]
pub enum ConfigBaudrate {
    /// 100 kHz
    Standard,

    /// 400 kHz
    Fast,
}
