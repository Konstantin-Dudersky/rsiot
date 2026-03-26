//! Компонент для взаимодействия с базой данных SurrealDB

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::{COMPONENT_NAME, Cmp};
pub use config::{Config, ConfigConnection, RequestInputConfig, RequestStartConfig};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

static DB: std::sync::LazyLock<surrealdb::Surreal<surrealdb::engine::any::Any>> =
    std::sync::LazyLock::new(surrealdb::Surreal::init);
