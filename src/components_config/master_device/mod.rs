//! Реализация опроса подчиненных устройств по любому протоколу - I2C, SPI, UART

mod buffer_bound;
mod device;
mod device_trait;
mod error;
mod fieldbus_request_with_index;
mod fieldbus_response_with_index;
mod request_response_bound;

pub use buffer_bound::BufferBound;
pub use device::*;
pub use device_trait::DeviceTrait;
pub use error::Error;
pub(crate) use fieldbus_request_with_index::FieldbusRequestWithIndex;
pub(crate) use fieldbus_response_with_index::FieldbusResponseWithIndex;
pub(crate) use request_response_bound::RequestResponseBound;

/// Тип Result
pub type Result<T> = std::result::Result<T, Error>;
