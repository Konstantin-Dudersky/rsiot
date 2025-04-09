//! Компонент HTTP-клиент для микроконтроллера ESP32

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use crate::components::shared_tasks::cmp_http_client::Error;
pub use component::Cmp;
pub use config::*;

type Result<T> = std::result::Result<T, Error>;
