//! Конфигурация Websocket-сервера
//!
//! Тестирование:
//!
//! ```bash
//! cargo test -p rsiot-components-config --doc websocket_server
//! ```

mod config;

pub use super::websocket_general::WebsocketMessage;
pub use config::{Config, FnInput, FnOutput};
