use std::time::{Duration, Instant};

use crate::components_config::master_device::RequestResponseBound;

use super::{I2cAddress, Operation};

// ANCHOR: FieldbusRequest
/// Структура отдельного запроса на коммуникацию по шине I2C
#[derive(Clone, Debug)]
pub struct FieldbusRequest {
    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Адрес устройства
    pub address: I2cAddress,

    /// Вид запроса.
    ///
    /// Необходим для правильной расшифровки ответа
    pub request_kind: u8,

    /// Массив операций
    pub operations: Vec<Operation>,
}
// ANCHOR: FieldbusRequest

impl FieldbusRequest {
    /// Создание запроса. Адрес задается позже
    pub fn new(
        address: I2cAddress,
        request_kind: impl Into<u8>,
        operations: Vec<Operation>,
    ) -> Self {
        Self {
            request_creation_time: Instant::now(),
            address,
            request_kind: request_kind.into(),
            operations,
        }
    }
}

impl RequestResponseBound for FieldbusRequest {
    fn request_duration(&self) -> Duration {
        Duration::default()
    }
}
