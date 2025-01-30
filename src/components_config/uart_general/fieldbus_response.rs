use std::{fmt::Debug, time::Instant};

use serde::de::DeserializeOwned;

use crate::{components_config::master_device::RequestResponseBound, serde_utils::postcard_serde};

/// Структура отдельного ответа при коммуникации по шине SPI
#[derive(Clone, Debug)]
pub struct FieldbusResponse {
    /// Адрес подчиненного устройства
    pub address: u8,

    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Ответ
    pub uart_response: Vec<u8>,
}

impl FieldbusResponse {
    /// Десериализация ответа
    pub fn response_deserialize<T>(&mut self) -> T
    where
        T: DeserializeOwned,
    {
        postcard_serde::deserialize_crc(&mut self.uart_response).unwrap()
    }
}

impl RequestResponseBound for FieldbusResponse {
    fn address(&self) -> u8 {
        self.address
    }

    fn set_address(&mut self, address: u8) {
        self.address = address
    }
}
