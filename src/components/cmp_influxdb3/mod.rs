//! Компонент для взаимодействия с InfluxDB

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use crate::components_config::influxdb3::LineProtocolItem;
pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
