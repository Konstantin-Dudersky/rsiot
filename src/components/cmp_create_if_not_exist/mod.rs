//! Создание компонентов, если они отсутсвуют в кеше
//!
//! Можно использовать, например, для создания настройк по-умолчанию, когда база данных пустая.

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
