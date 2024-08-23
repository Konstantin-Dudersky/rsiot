//! Компонент для взаимодействия с базой данных SurrealDB

use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::sync::Mutex;

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::Cmp;
pub use config::{Config, InputConfig};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
type DbClient = Arc<Mutex<Surreal<Client>>>;
