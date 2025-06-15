use std::time::{Duration, Instant};

use crate::components_config::master_device::RequestResponseBound;

/// Структура отдельного запроса на коммуникацию по шине SPI
#[derive(Clone, Debug)]
pub struct FieldbusRequest {
    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Адрес устройства
    pub address: u8,

    /// Вид запроса.
    ///
    /// Необходим для правильной расшифровки ответа
    pub request_kind: u8,

    /// Массив операций
    pub operations: Vec<Operation>,
}

impl FieldbusRequest {
    /// Создание запроса. Адрес задается позже
    pub fn new(address: u8, request_kind: impl Into<u8>, operations: Vec<Operation>) -> Self {
        Self {
            request_creation_time: Instant::now(),
            address,
            request_kind: request_kind.into(),
            operations,
        }
    }
}

/// Виды операций
#[derive(Clone, Debug)]
pub enum Operation {
    /// Задержка между операциями
    Delay {
        /// Значение задержки
        delay: Duration,
    },

    /// Запрос записи и  чтения. Вложенные данные - количество байт для чтения
    WriteRead {
        /// Данные для записи
        write_data: Vec<u8>,
        /// Количество байт для чтения
        read_size: u8,
    },

    /// Запрос записи
    Write {
        /// Данные для записи
        write_data: Vec<u8>,
    },

    /// Запрос чтения
    Read {
        /// Количество байт для чтения
        read_size: u8,
    },
}

impl RequestResponseBound for FieldbusRequest {}
