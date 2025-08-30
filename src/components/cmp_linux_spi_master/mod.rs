//! Опрос устройств по интерфейсу SPI

mod component;
mod config;
mod error;
mod fn_process;

pub use crate::components_config::spi_master::ConfigDeviceSpiMode;
pub use component::{COMPONENT_NAME, Cmp};
pub use config::{Config, ConfigDevicesCommSettings, LinuxDevice};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
