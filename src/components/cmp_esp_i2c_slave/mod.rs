//! Компонент для подключения контроллера ESP32 как I2C slave.

// TODO: убрать зависимость от postcard

mod buffer_data;
mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use buffer_data::BufferData;
pub use component::{COMPONENT_NAME, Cmp};
pub use config::*;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
