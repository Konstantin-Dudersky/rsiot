//! Структуры данных для реализации мастера шины I2C

mod fieldbus_request;
mod fieldbus_response;
mod i2c_address;
mod operation;

pub use {
    fieldbus_request::FieldbusRequest, fieldbus_response::FieldbusResponse,
    i2c_address::I2cAddress, operation::Operation,
};
