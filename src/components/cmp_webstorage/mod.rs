//! Хранение и загрузка сообщений используя LocalStorage или SessionStorage браузера

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigKind};
pub use error::Error;

type Result<T> = std::result::Result<T, error::Error>;
