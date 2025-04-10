//! Пример протокола для связи через UART
//!
//! Можно использовать как есть. В клиентском коде при необходимости можно создать другой протокол.

mod crc_alg;
mod error;
mod protocol;
mod uart_packet;

use super::{FieldbusRequest, FieldbusResponse};

pub use error::Error;
pub use protocol::Protocol;
pub use uart_packet::UartPacket;
