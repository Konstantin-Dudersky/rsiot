use std::{fmt::Debug, time::Instant};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    components_config::master_device::RequestResponseBound,
    serde_utils::postcard_serde::{self, serialize_nocrc_vec},
};

use super::uart_message::UartMessage;

/// Структура отдельного запроса на коммуникацию по шине SPI
#[derive(Clone, Debug)]
pub struct FieldbusRequest {
    /// Адрес подчиненного устройства
    pub address: u8,

    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Данные для передачи по uart
    payload: Vec<u8>,
}

impl FieldbusRequest {
    /// Создание запроса. Адрес задается позже
    pub fn new(uart_request: impl Serialize) -> Self {
        Self {
            address: Default::default(),
            request_creation_time: Instant::now(),
            payload: serialize_nocrc_vec(&uart_request).unwrap(),
        }
    }

    /// Десериализация запроса
    pub fn get_payload<T>(&mut self) -> Result<T, postcard_serde::Error>
    where
        T: DeserializeOwned,
    {
        postcard_serde::deserialize_nocrc(&mut self.payload)
    }

    /// Восстановить запрос из буфера передачи по сети
    pub fn from_read_buffer(read_buf: &mut [u8]) -> Result<Self, postcard_serde::Error> {
        let uart_message: UartMessage = postcard_serde::deserialize_crc(read_buf)?;
        let fieldbus_response = Self {
            address: uart_message.address,
            request_creation_time: Instant::now(),
            payload: uart_message.payload,
        };
        Ok(fieldbus_response)
    }

    /// Подготовить запрос для передачи по сети
    pub fn to_write_buffer<const MESSAGE_LEN: usize>(
        self,
    ) -> Result<[u8; MESSAGE_LEN], postcard_serde::Error> {
        let uart_message = UartMessage {
            address: self.address,
            payload: self.payload,
        };
        postcard_serde::serialize_crc_arr(&uart_message)
    }
}

impl RequestResponseBound for FieldbusRequest {
    fn address(&self) -> u8 {
        self.address
    }

    fn set_address(&mut self, address: u8) {
        self.address = address
    }
}
