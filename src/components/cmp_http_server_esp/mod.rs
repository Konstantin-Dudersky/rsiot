//! HTTP-сервер в микроконтроллере ESP32

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigCmpPlcData};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
