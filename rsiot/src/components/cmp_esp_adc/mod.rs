//! Компонент для чтения аналоговых сигналов микроконтроллера ESP

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigInput, ConfigItem};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
