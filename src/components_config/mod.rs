//! Структуры для конфигурации компонентов.
//!
//! Конфигурация описывается в элементах языка Rust, не зависит от конкретных коммуникационных
//! библиотек. Конкретные реализации компонентов импортируют этот крейт.

#[cfg(any(feature = "cmp_linux_can", feature = "cmp_esp"))]
pub mod can_general;

pub mod http_client;

pub mod http_general;

#[cfg(any(feature = "cmp_http_server", feature = "cmp_esp"))]
pub mod http_server;

#[cfg(feature = "executor")]
pub mod i2c_master;

#[cfg(feature = "cmp_influxdb")]
pub mod influxdb3;

#[cfg(feature = "cmp_modbus_client")]
pub mod modbus_client;

#[cfg(any(feature = "cmp_mqtt_client", feature = "cmp_esp"))]
pub mod mqtt_client;

#[cfg(feature = "executor")]
pub mod uart_general;

#[cfg(any(feature = "cmp_esp", feature = "cmp_linux_uart_master",))]
pub mod uart_master;

#[cfg(feature = "executor")]
pub mod spi_master;

#[cfg(feature = "executor")]
pub mod master_device;

#[cfg(any(
    feature = "cmp_websocket_client",
    feature = "cmp_websocket_client_wasm"
))]
pub mod websocket_client;

pub mod websocket_general;

#[cfg(feature = "cmp_websocket_server")]
pub mod websocket_server;
