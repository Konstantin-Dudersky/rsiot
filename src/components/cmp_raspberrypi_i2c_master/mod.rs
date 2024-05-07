//! Компонент cmp_raspberrypi_i2c_master

mod component;
mod config;
mod error;
mod fn_process;
mod rsiot_i2c_driver;

pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
