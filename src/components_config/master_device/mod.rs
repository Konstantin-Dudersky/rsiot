//! Реализация опроса подчиненных устройств по любому протоколу - I2C, SPI, UART

mod buffer_bound;
mod device;
mod device_trait;
mod error;
mod request_response_bound;

pub use buffer_bound::BufferBound;
pub use device::*;
pub use device_trait::DeviceTrait;
pub use error::Error;
pub use request_response_bound::RequestResponseBound;

pub type Result<T> = std::result::Result<T, Error>;
