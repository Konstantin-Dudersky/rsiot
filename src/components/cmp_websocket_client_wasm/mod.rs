//!  Компонент клиента websocket для WASM

mod component;
mod config;
mod fn_process;
mod tasks;

pub use component::Cmp;
pub use config::Config;

use super::cmp_websocket_client_general::{Error, Result};
