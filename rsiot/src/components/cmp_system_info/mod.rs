//! Компонент получает системную информацию

mod component;
mod config;
mod error;
mod fn_process;
mod system_info;

pub use component::Cmp;
pub use config::Config;
pub use error::Error;
pub use system_info::{SystemInfo, SystemInfoDisk, SystemInfoMemory, SystemInfoNetwork};

type Result<T> = std::result::Result<T, Error>;
