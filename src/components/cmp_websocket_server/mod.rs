//! Компонент для подключения через websocket server.
//!
//! Перенаправляет поток входящих сообщений подключенным вебсокет-клиентам.
//!
//! При подключении клиенту отправляются все актуальные сообщения из кеша, далее только новые.
//! Используется библиотека [tokio-tungstenite](https://crates.io/crates/tokio-tungstenite).
//!
//! # Пример
//!
//! ## Файл `shared::ws_client_server/client_to_server.rs`
//! ```
#![doc = include_str!("doc/client_to_server.rs")]
//!```
//!
//! ## Файл `shared::ws_client_server/server_to_client.rs`
//! ```
#![doc = include_str!("doc/server_to_client.rs")]
//!```
//!
//! ## Файл `cmp_websocket_server/mod.rs`
//! ```
#![doc = include_str!("doc/new.rs")]
//! ```

mod component;
mod config;
mod errors;
mod fn_process;
// mod handle_ws_connection;
#[cfg(feature = "rustdoc")]
mod doc;
mod tasks;

pub use component::{COMPONENT_NAME, Cmp};
pub use config::Config;
pub use errors::Error;

type Result<T> = std::result::Result<T, errors::Error>;

type ServerToClientCache<TServerToClient> =
    std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, TServerToClient>>>;
