use std::{fmt::Debug, time::Instant};

use crate::components_config::master_device::RequestResponseBound;

// ANCHOR: FieldbusResponse
/// Структура отдельного ответа при коммуникации по шине UART
#[derive(Clone, Debug)]
pub struct FieldbusResponse {
    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Ответ
    pub packet: Vec<u8>,
}
// ANCHOR: FieldbusResponse

impl FieldbusResponse {
    /// Создание ответа
    pub fn new(packet: Vec<u8>) -> Self {
        Self {
            request_creation_time: Instant::now(),
            packet,
        }
    }

    /// Подготовить ответ для передачи по сети
    pub fn to_write_buffer(self) -> Vec<u8> {
        self.packet
    }
}

impl RequestResponseBound for FieldbusResponse {}
