use std::{fmt::Debug, time::Instant};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    components_config::master_device::RequestResponseBound,
    serde_utils::postcard_serde::{self, serialize_nocrc},
};

use super::uart_message::UartMessage;

/// Структура отдельного ответа при коммуникации по шине SPI
#[derive(Clone, Debug)]
pub struct UartResponse {
    /// Адрес подчиненного устройства
    pub address: u8,

    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Ответ
    payload: Vec<u8>,
}

impl UartResponse {
    /// Создание ответа. Адрес задается позже
    pub fn new(uart_request: impl Serialize) -> Self {
        Self {
            address: Default::default(),
            request_creation_time: Instant::now(),
            payload: serialize_nocrc(&uart_request).unwrap(),
        }
    }

    /// Десериализация ответа
    pub fn get_payload<T>(&mut self) -> Result<T, postcard_serde::Error>
    where
        T: DeserializeOwned,
    {
        postcard_serde::deserialize_nocrc(&mut self.payload)
    }

    /// Установить время создания запроса
    ///
    /// Время не передается по сети, поэтому необходимо устанавливать вручную
    pub fn set_request_creation_time(&mut self, request_creation_time: Instant) {
        self.request_creation_time = request_creation_time;
    }

    /// Восстановить ответ из буфера передачи по сети
    pub fn from_read_buffer(read_buf: &mut [u8]) -> Result<Self, postcard_serde::Error> {
        let uart_message: UartMessage = postcard_serde::deserialize_crc(read_buf)?;
        let fieldbus_response = Self {
            address: uart_message.address,
            request_creation_time: Instant::now(),
            payload: uart_message.payload,
        };
        Ok(fieldbus_response)
    }

    /// Подготовить ответ для передачи по сети
    pub fn to_write_buffer(self) -> Result<Vec<u8>, postcard_serde::Error> {
        let uart_message = UartMessage {
            address: self.address,
            payload: self.payload,
        };
        postcard_serde::serialize_crc(&uart_message)
    }
}

impl RequestResponseBound<u8> for UartResponse {
    fn address(&self) -> u8 {
        self.address
    }

    fn set_address(&mut self, address: u8) {
        self.address = address
    }
}
