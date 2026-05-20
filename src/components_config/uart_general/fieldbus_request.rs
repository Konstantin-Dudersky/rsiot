use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::components_config::master_device::RequestResponseBound;

// ANCHOR: FieldbusRequest
/// Структура отдельного запроса на коммуникацию по шине UART
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

    /// Данные для передачи по uart
    pub packet: Vec<u8>,
}
// ANCHOR: FieldbusRequest

impl FieldbusRequest {
    /// Создание запроса
    pub fn new(request_kind: impl Into<u8>, packet: Vec<u8>) -> Self {
        Self {
            request_creation_time: Instant::now(),
            request_kind: request_kind.into(),
            packet,
        }
    }

    // /// Восстановить запрос из буфера передачи по сети
    // pub fn from_read_buffer(read_buf: &[u8]) -> Self {
    //     Self {
    //         request_creation_time: Instant::now(),
    //         packet: read_buf.to_vec(),
    //     }
    // }

    // /// Подготовить запрос для передачи по сети
    // pub fn to_write_buffer(self) -> Vec<u8> {
    //     self.packet
    // }
}

impl RequestResponseBound for FieldbusRequest {
    fn request_duration(&self) -> Duration {
        Duration::default()
    }
}
