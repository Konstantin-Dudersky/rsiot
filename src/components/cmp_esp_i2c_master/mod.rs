//! Компнент опроса по протоколу i2c для микроконтроллера ESP32

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
