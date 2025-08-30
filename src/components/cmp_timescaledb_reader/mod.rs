//! Компонент cmp_timescaledb_reader

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::{COMPONENT_NAME, Cmp};
pub use config::{Config, ConfigItem};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
