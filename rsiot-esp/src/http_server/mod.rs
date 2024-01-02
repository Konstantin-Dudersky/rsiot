//! HTTP-сервер из ESP-IDF ограничен только одним экземпляром. Например, если запусить
//! Websocket-server с бесконечным циклом для рассылки сообщений клиентам, то сервер не будет
//! обрабатывать HTTP-запросы. Поэтому, хотя Websocket-сервер есть, он мало пригоден для
//! использования.

mod config;
mod fn_process;
mod new;

/// HTTP-сервер, работающий на ESP32
pub mod cmp_http_server_esp {
    pub use super::config::Config;
    pub use super::new::new;
}
