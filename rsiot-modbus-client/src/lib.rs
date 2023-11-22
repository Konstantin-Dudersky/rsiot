//! Modbus клиент
#![doc=include_str!("../../doc/component-modbus-client.svg")]
//!

mod errors;
mod start_modbus_client;
mod types;

pub use errors::Errors;
pub use start_modbus_client::start_modbus_client;
pub use types::Result_;
