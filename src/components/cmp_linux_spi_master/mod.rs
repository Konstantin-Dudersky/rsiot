//! Шаблон компонента

mod component;
mod config;
mod error;
mod fn_process;

pub use crate::components_config::spi_master::ConfigDeviceSpiMode;
pub use component::Cmp;
pub use config::{Config, ConfigDevicesCommSettings};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
