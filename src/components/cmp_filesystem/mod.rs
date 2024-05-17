//! Компонент для сохранения данных в файловой системе и загрузки данных из файловой системы

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigInput};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
