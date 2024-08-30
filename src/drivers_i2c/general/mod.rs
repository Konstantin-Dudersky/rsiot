//! Общий slave I2C

mod config;
mod device;
mod error;

pub use config::*;
pub use device::Device;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
