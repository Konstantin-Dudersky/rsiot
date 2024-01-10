//! Компонент для подключения через websocket server.
//!
//! Перенаправляет поток входящих сообщений подключенным вебсокет-клиентам.
//!
//! При подключении клиенту отправляются все актуальные сообщения из кеша, далее только новые.
//! Используется библиотека [tokio-tungstenite](https://crates.io/crates/tokio-tungstenite).
//!
//! # Пример
//!
//! ```
#![doc = include_str!("../examples/example1.rs")]
//! ```

mod async_task_utils;
mod component;
mod config;
mod errors;
mod fn_process;
mod handle_ws_connection;

/// Компонент для подключения через websocket server.
pub mod cmp_websocket_server {
    pub use crate::{component::Cmp, config::Config};
}
