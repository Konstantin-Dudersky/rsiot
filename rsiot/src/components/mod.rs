mod components_config;

#[cfg(feature = "components")]
pub use rsiot_extra_components::*;

#[cfg(feature = "cmp_auth")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_auth;

#[cfg(feature = "cmp_http_client")]
pub mod cmp_http_client;

#[cfg(feature = "cmp_http_client_wasm")]
pub mod cmp_http_client_wasm;

#[cfg(feature = "http-server")]
pub use rsiot_http_server as cmp_http_server;

#[cfg(feature = "influxdb")]
pub use rsiot_influxdb as cmp_influxdb;

#[cfg(feature = "cmp_leptos")]
#[cfg(target_arch = "wasm32")]
pub mod cmp_leptos;

#[cfg(feature = "cmp_modbus_client")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_modbus_client;

#[cfg(feature = "plc")]
pub use rsiot_plc as cmp_plc;

#[cfg(feature = "cmp_redis_client")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_redis_client;

#[cfg(feature = "surrealdb")]
pub use rsiot_surrealdb as cmp_surrealdb;

#[cfg(feature = "timescaledb-storing")]
pub use rsiot_timescaledb_storing::cmp_timescaledb_storing;

#[cfg(feature = "cmp_websocket_client")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_websocket_client;

#[cfg(feature = "cmp_websocket_client_wasm")]
#[cfg(target_arch = "wasm32")]
pub mod cmp_websocket_client_wasm;

#[cfg(feature = "cmp_websocket_server")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_websocket_server;

#[cfg(feature = "cmp_webstorage")]
#[cfg(target_arch = "wasm32")]
pub mod cmp_webstorage;
