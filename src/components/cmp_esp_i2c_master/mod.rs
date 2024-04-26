//! Компнент опроса по протоколу i2c для микроконтроллера ESP32

mod component;
mod config;
mod error;
mod fn_process;
mod rsiot_i2c_driver;

pub use crate::drivers_i2c::I2cDevices;
pub use component::Cmp;
pub use config::{Config, ConfigBaudrate};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
