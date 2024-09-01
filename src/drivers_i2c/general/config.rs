use std::time::Duration;

use super::super::I2cSlaveAddress;

/// Функция обработки ответа
pub type FnResponse = fn(usize, &mut [u8]) -> Result<(), anyhow::Error>;

/// Конфигурация
#[derive(Clone)]
pub struct Config {
    /// Адрес
    pub address: I2cSlaveAddress,

    /// Массив запросов
    pub requests: Vec<ConfigRequestKind>,

    /// Период выполнения запросов
    pub period: Duration,

    /// Функция обработки ответа
    pub fn_response: FnResponse,

    /// Тайм-аут запроса
    pub timeout: Duration,
}

/// Конфигурация одного запроса
#[derive(Clone)]
pub enum ConfigRequestKind {
    /// Запрос read
    Read {
        /// Кол-во байт чтения
        response_size: usize,
    },
    /// Запрос write
    Write {
        /// Данные для записи
        request: Vec<u8>,
    },
    /// Запрос write_read
    WriteRead {
        /// Данные для записи
        request: Vec<u8>,
        /// Кол-во байт чтения
        response_size: usize,
    },
}
