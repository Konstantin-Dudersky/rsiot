#![allow(clippy::module_inception)]

use std::time::Instant;

use serde::{de::DeserializeOwned, Serialize};

use crate::serde_utils::{SerdeAlg, SerdeAlgKind};

use super::{crc_alg::CrcAlg, Error, FieldbusRequest, FieldbusResponse, UartPacket};

/// Протокол передачи
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Protocol {
    /// Адрес устройства
    pub address: u8,
    /// Алгоритм сериализации / десериализации
    pub serde_alg: SerdeAlg,
}
impl Protocol {
    /// Создание протокола
    pub fn new(address: u8) -> Self {
        Self {
            address,
            serde_alg: SerdeAlg::new(SerdeAlgKind::Postcard),
        }
    }

    /// Сериализация запроса
    pub fn serialize_request<TData>(&self, data: TData) -> Result<FieldbusRequest, Error>
    where
        TData: Serialize,
    {
        let uart_message = UartPacket {
            // transaction_id: 0,
            address: self.address,
            data,
        };

        let mut payload = self.serde_alg.serialize(&uart_message)?;
        CrcAlg::calculate(&mut payload);

        let uart_request = FieldbusRequest {
            request_creation_time: Instant::now(),
            packet: payload,
        };
        Ok(uart_request)
    }

    /// Десериализация запроса
    pub fn deserialize_request<TData>(
        &self,
        request: FieldbusRequest,
    ) -> Result<UartPacket<TData>, Error>
    where
        TData: DeserializeOwned,
    {
        let payload = CrcAlg::check(&request.packet)?;
        let uart_packet = self.serde_alg.deserialize(payload)?;
        Ok(uart_packet)
    }

    /// Сериализация ответа
    pub fn serialize_response<TData>(&self, data: TData) -> Result<FieldbusResponse, Error>
    where
        TData: Serialize,
    {
        let uart_message = UartPacket {
            address: self.address,
            data,
        };

        let mut payload = self.serde_alg.serialize(&uart_message)?;
        CrcAlg::calculate(&mut payload);

        let uart_response = FieldbusResponse {
            request_creation_time: Instant::now(),
            packet: payload,
        };
        Ok(uart_response)
    }

    /// Десериализация ответа
    pub fn deserialize_response<TData>(
        &self,
        response: FieldbusResponse,
    ) -> Result<UartPacket<TData>, Error>
    where
        TData: DeserializeOwned,
    {
        let payload = CrcAlg::check(&response.packet)?;
        let uart_packet = self.serde_alg.deserialize(payload)?;
        Ok(uart_packet)
    }
}
