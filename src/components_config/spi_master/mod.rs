//! Структуры данных для реализации мастера шины SPI

mod config_device_spi_mode;
mod fieldbus_request;
mod fieldbus_response;
mod operation;

pub use {
    config_device_spi_mode::ConfigDeviceSpiMode, fieldbus_request::FieldbusRequest,
    fieldbus_response::FieldbusResponse, operation::Operation,
};
