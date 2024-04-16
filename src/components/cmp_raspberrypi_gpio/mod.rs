//! Компонент для работы с GPIO Raspberry Pi

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigInput, ConfigOutput};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
