//! Структуры данных для реализации мастера шины SPI

mod fieldbus_request;
mod fieldbus_response;

pub use fieldbus_request::{FieldbusRequest, Operation};
pub use fieldbus_response::FieldbusResponse;

use super::master_device::AddressBound;

impl AddressBound for u8 {}
