//! Структуры данных для реализации мастера шины SPI

mod config_device_spi_mode;
mod fieldbus_request;
mod fieldbus_response;

pub use config_device_spi_mode::ConfigDeviceSpiMode;
pub use fieldbus_request::{FieldbusRequest, Operation};
pub use fieldbus_response::FieldbusResponse;
