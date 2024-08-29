use std::time::Duration;

use super::super::I2cSlaveAddress;

pub type FnResponse = fn(usize, &mut [u8]) -> Result<(), String>;

/// Конфигурация
#[derive(Clone)]
pub struct Config {
    /// Адрес
    pub address: I2cSlaveAddress,

    pub requests: Vec<ConfigRequestKind>,

    pub period: Duration,

    pub fn_response: FnResponse,

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
