#[cfg(feature = "components")]
pub mod cmp_add_input_stream;

pub mod cmp_add_output_stream;

#[cfg(feature = "cmp_auth")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_auth;

pub mod cmp_derive;

pub mod cmp_external_fn_process;

pub mod cmp_inject_periodic;

#[cfg(feature = "cmp_http_client")]
pub mod cmp_http_client;

#[cfg(feature = "cmp_http_client_wasm")]
#[cfg(target_arch = "wasm32")]
pub mod cmp_http_client_wasm;

#[cfg(feature = "cmp_http_server")]
pub mod cmp_http_server;

#[cfg(feature = "cmp_influxdb")]
pub mod cmp_influxdb;

#[cfg(feature = "cmp_leptos")]
#[cfg(target_arch = "wasm32")]
pub mod cmp_leptos;

pub mod cmp_logger;

#[cfg(feature = "cmp_modbus_client")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_modbus_client;

#[cfg(feature = "cmp_plc")]
pub mod cmp_plc;

#[cfg(feature = "cmp_redis_client")]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod cmp_redis_client;

#[cfg(feature = "cmp_storage_esp")]
pub mod cmp_storage_esp;

#[cfg(feature = "cmp_surrealdb")]
pub mod cmp_surrealdb;

#[cfg(feature = "cmp_timescaledb")]
pub mod cmp_timescaledb;

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
