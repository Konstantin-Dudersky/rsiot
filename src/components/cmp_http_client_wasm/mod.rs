//! HTTP-клиент для платформы WASM

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::config as http_client_config;

type Result<T> = std::result::Result<T, error::Error>;
