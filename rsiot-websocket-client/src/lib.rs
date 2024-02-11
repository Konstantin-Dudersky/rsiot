#![cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]

mod component;
mod config;
mod error;
mod fn_process;

/// Компонент для подключения через websocket server.
pub mod cmp_websocket_client {
    pub use crate::{component::Cmp, config::Config};
}
