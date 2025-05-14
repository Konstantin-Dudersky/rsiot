//! Структуры данных для работы клиента Modbus

mod config;
mod fieldbus_request;
mod fieldbus_response;

pub use {
    config::{ClientType, Config, ConfigDevicesCommSettings},
    fieldbus_request::{FieldbusRequest, RequestContent},
    fieldbus_response::{FieldbusResponse, ResponseContent},
};
