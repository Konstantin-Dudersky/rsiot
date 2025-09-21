//! Клиент MQTT микроконтроллера ESP32

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::{COMPONENT_NAME, Cmp};
pub use config::*;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
