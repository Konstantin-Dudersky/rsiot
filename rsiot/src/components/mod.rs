mod components_config;

#[cfg(feature = "components")]
pub use rsiot_extra_components::*;

#[cfg(feature = "cmp_auth")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_auth;

#[cfg(feature = "http-client")]
pub use rsiot_http_client::cmp_http_client;

#[cfg(feature = "http-server")]
pub use rsiot_http_server as cmp_http_server;

#[cfg(feature = "influxdb")]
pub use rsiot_influxdb as cmp_influxdb;

#[cfg(feature = "leptos")]
pub use rsiot_leptos as cmp_leptos;

#[cfg(feature = "modbus-client")]
pub use rsiot_modbus_client::cmp_modbus_client;

#[cfg(feature = "plc")]
pub use rsiot_plc as cmp_plc;

#[cfg(feature = "redis-client")]
pub use rsiot_redis_client as cmp_redis_client;

#[cfg(feature = "surrealdb")]
pub use rsiot_surrealdb as cmp_surrealdb;

#[cfg(feature = "timescaledb-storing")]
pub use rsiot_timescaledb_storing::cmp_timescaledb_storing;

#[cfg(feature = "websocket-client")]
pub use rsiot_websocket_client::cmp_websocket_client;

#[cfg(feature = "websocket-client-wasm")]
pub use rsiot_websocket_client_wasm::cmp_websocket_client_wasm;

#[cfg(feature = "cmp_websocket_server")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_websocket_server;

#[cfg(feature = "cmp_webstorage")]
#[cfg(target_arch = "wasm32")]
pub mod cmp_webstorage;
