//! Структуры для конфигурации компонентов.
//!
//! Конфигурация описывается в элементах языка Rust, не зависит от конкретных коммуникационных
//! библиотек. Конкретные реализации компонентов импортируют этот крейт.

pub mod http_client;
pub mod http_server;
pub mod influxdb_v2;
pub mod redis_client;
pub mod timescaledb_storing;
pub mod websocket_client;
pub mod websocket_common;
