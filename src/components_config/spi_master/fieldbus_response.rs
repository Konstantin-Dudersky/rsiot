use std::time::Instant;

use crate::components_config::master_device::RequestResponseBound;

/// Структура отдельного ответа при коммуникации по шине SPI
#[derive(Clone, Debug)]
pub struct FieldbusResponse {
    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Вид запроса.
    ///
    /// Необходим для правильной расшифровки ответа
    pub request_kind: u8,

    /// Данные, содержащие ответы
    pub payload: Vec<Vec<u8>>,
}

impl RequestResponseBound for FieldbusResponse {}
