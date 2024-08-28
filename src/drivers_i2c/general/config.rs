use std::time::Duration;

use super::super::I2cSlaveAddress;

/// Конфигурация
#[derive(Clone)]
pub struct Config {
    /// Адрес
    pub address: I2cSlaveAddress,

    pub requests: Vec<ConfigRequestKind>,

    pub period: Duration,

    pub fn_response: fn(usize, Vec<u8>),

    /// Тайм-аут запроса
    pub timeout: Duration,
}

#[derive(Clone)]

pub enum ConfigRequestKind {
    Read {
        response_size: usize,
    },
    Write {
        request: Vec<u8>,
    },
    WriteRead {
        request: Vec<u8>,
        response_size: usize,
    },
}
