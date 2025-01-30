use std::{fmt::Debug, time::Instant};

use serde::Serialize;

use crate::{
    components_config::master_device::RequestResponseBound,
    serde_utils::postcard_serde::serialize_nocrc_vec,
};

/// Структура отдельного запроса на коммуникацию по шине SPI
#[derive(Clone, Debug)]
pub struct FieldbusRequest {
    /// Адрес подчиненного устройства
    pub address: u8,

    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Запрос
    pub uart_request: Vec<u8>,
}

impl FieldbusRequest {
    /// Создание запроса. Адрес задается позже
    pub fn new(uart_request: impl Serialize) -> Self {
        Self {
            address: Default::default(),
            request_creation_time: Instant::now(),
            uart_request: serialize_nocrc_vec(&uart_request).unwrap(),
        }
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
