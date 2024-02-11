#![cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{msg_into_line_protocol, Config, DataPointVaueType, LineProtocolItem};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

// TODO - после выхода InfluxDB 3.0 (май 2024) пересмотреть
