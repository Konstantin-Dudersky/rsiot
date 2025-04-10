use std::time::{Duration, Instant};

use crate::components_config::master_device::RequestResponseBound;

/// Структура отдельного запроса на коммуникацию по шине SPI
#[derive(Clone, Debug)]
pub struct FieldbusRequest {
    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Вид запроса.
    ///
    /// Необходим для правильной расшифровки ответа
    pub request_kind: u8,

    /// Массив операций
    pub operations: Vec<Operation>,
}

impl FieldbusRequest {
    /// Создание запроса. Адрес задается позже
    pub fn new(request_kind: impl Into<u8>, operations: Vec<Operation>) -> Self {
        Self {
            request_creation_time: Instant::now(),
            request_kind: request_kind.into(),
            operations,
        }
    }
}

/// Виды операций
#[derive(Clone, Debug)]
pub enum Operation {
    /// Задержка между операциями
    Delay(Duration),

    /// Запрос записи и  чтения. Вложенные данные - количество байт для чтения
    WriteRead(Vec<u8>, u8),

    /// Запрос записи. Вложенные данные - массив байт для записи
    Write(Vec<u8>),
}

impl RequestResponseBound for FieldbusRequest {}
