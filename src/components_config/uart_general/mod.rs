//! Общие структуры данных для устройств, обменивающихся данными по UART

mod baudrate;
mod data_bits;
mod fieldbus_request;
mod fieldbus_response;
mod parity;
mod stop_bits;
// mod uart_message;
// mod uart_message_raw;

pub use baudrate::Baudrate;
pub use data_bits::DataBits;
pub use fieldbus_request::FieldbusRequest;
pub use fieldbus_response::FieldbusResponse;
pub use parity::Parity;
pub use stop_bits::StopBits;
// pub use uart_message::UartMessage;
// pub use uart_message_raw::UartMessageRaw;
