//! Структуры для конфигурации компонентов.
//!
//! Конфигурация описывается в элементах языка Rust, не зависит от конкретных коммуникационных
//! библиотек. Конкретные реализации компонентов импортируют этот крейт.

#[cfg(any(
    feature = "cmp_esp",
    feature = "cmp_http_client",
    feature = "cmp_http_client_wasm",
))]
pub mod http_client;

#[cfg(feature = "cmp_influxdb")]
pub mod influxdb_v2;

#[cfg(feature = "cmp_modbus_client")]
pub mod modbus_client;

#[cfg(any(feature = "cmp_mqtt_client", feature = "cmp_esp"))]
pub mod mqtt_client;

#[cfg(any(feature = "cmp_http_server", feature = "cmp_esp"))]
pub mod http_server;

#[cfg(feature = "cmp_redis_client")]
pub mod redis_client;

#[cfg(any(
    feature = "cmp_websocket_client",
    feature = "cmp_websocket_client_wasm"
))]
pub mod websocket_client;

#[cfg(feature = "cmp_timescaledb")]
pub mod timescaledb;

#[cfg(feature = "cmp_websocket_server")]
pub mod websocket_server;

pub mod uart_general;

#[cfg(any(feature = "cmp_esp", feature = "cmp_linux_uart_master",))]
pub mod uart_master;

#[cfg(any(feature = "cmp_esp"))]
mod spi_master;
