//! Компонент для взаимодействия с базой данных SurrealDB

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, InputConfig};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
