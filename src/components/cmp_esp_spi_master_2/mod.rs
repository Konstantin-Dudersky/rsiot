//! Компонент для работы с подчиненными устройствами по шине SPI

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
