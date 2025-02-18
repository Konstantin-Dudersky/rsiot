//! Шаблон компонента

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
