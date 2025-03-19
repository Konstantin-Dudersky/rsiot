//! HTTP-сервер в микроконтроллере ESP32

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use crate::components_config::http_server::{
    GetEndpoint, GetEndpointConfig, PutEndpoint, PutEndpointConfig,
};
pub use component::Cmp;
pub use config::Config;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
