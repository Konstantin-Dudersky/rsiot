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
    pub response_content: ResponseContent,
}

impl RequestResponseBound for FieldbusResponse {}

impl FieldbusResponse {
    /// Создать ответ из запроса
    pub fn from_request(
        request: super::FieldbusRequest,
        response_content: ResponseContent,
    ) -> Self {
        Self {
            request_creation_time: request.request_creation_time,
            request_kind: request.request_kind,
            response_content,
        }
    }
}

/// Ответ от устройства
#[derive(Clone, Debug)]
pub enum ResponseContent {
    /// Массив слов
    WordVector(Vec<u16>),

    /// Массив бит
    BitVector(Vec<bool>),

    /// Без ответа
    Unit,
}
