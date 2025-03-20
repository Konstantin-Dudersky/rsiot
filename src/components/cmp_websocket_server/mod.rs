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
// #![doc = include_str!("../../../examples/cmp_websocket_server.rs")]
//! ```

#![cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]

mod async_task_utils;
mod component;
mod config;
mod errors;
mod fn_process;
// mod handle_ws_connection;
mod tasks;

pub use component::Cmp;
pub use config::Config;
pub use errors::Error;

type Result<T> = std::result::Result<T, errors::Error>;

type ServerToClientCache<TServerToClient> =
    std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, TServerToClient>>>;
