//! Структуры данных для реализации мастера шины I2C

mod fieldbus_request;
mod fieldbus_response;
mod operation;

pub use {
    fieldbus_request::FieldbusRequest, fieldbus_response::FieldbusResponse, operation::Operation,
};
