mod device_base;
mod device_trait;
pub mod devices;
mod error;

use super::uart_general::{UartMessage, UartMessageRaw};

pub use device_trait::DeviceTrait;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
