//! Хранение и загрузка сообщений используя LocalStorage браузера

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigKind};
pub use error::Error;

type Result<T> = std::result::Result<T, error::Error>;
