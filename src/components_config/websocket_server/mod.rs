//! Конфигурация Websocket-сервера
//!
//! Тестирование:
//!
//! ```bash
//! cargo test -p rsiot-components-config --doc websocket_server
//! ```

mod config;
mod ws_data;

pub use {
    super::websocket_general::WebsocketMessage,
    config::{Config, FnInput, FnOutput},
    ws_data::WsData,
};
