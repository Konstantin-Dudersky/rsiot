//! Общие структуры данных для устройств, обменивающихся данными по UART

mod baudrate;
mod calculate_transmission_time;
mod data_bits;
mod parity;
mod stop_bits;
mod uart_message;
mod uart_request;
mod uart_response;

pub use baudrate::Baudrate;
pub use calculate_transmission_time::{calculate_transmission_time, data_rate};
pub use data_bits::DataBits;
pub use parity::Parity;
pub use stop_bits::StopBits;
pub use uart_request::UartRequest;
pub use uart_response::UartResponse;
