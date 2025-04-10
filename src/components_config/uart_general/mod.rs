//! Общие структуры данных для устройств, обменивающихся данными по UART

mod baudrate;
mod calculate_transmission_time;
mod data_bits;
mod fieldbus_request;
mod fieldbus_response;
mod parity;
pub mod protocol;
mod stop_bits;

pub use baudrate::Baudrate;
pub use calculate_transmission_time::{calculate_transmission_time, data_rate};
pub use data_bits::DataBits;
pub use fieldbus_request::FieldbusRequest;
pub use fieldbus_response::FieldbusResponse;
pub use parity::Parity;
pub use stop_bits::StopBits;
