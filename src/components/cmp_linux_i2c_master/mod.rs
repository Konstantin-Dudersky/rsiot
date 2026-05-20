//! Опрос устройств по интерфейсу I2C

mod component;
mod config;
mod error;
mod fn_process;

pub use {
    crate::components_config::i2c_master::*,
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
};

type Result<T> = std::result::Result<T, Error>;
