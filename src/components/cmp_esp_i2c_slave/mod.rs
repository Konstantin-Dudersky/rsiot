//! Компонент для подключения контроллера ESP32 как I2C slave.

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::Cmp;
pub use config::*;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
