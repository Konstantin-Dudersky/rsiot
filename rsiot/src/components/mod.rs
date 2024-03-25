//! Компоненты

#[allow(dead_code)]
#[allow(unused_imports)]
mod _cmp_template;

pub mod cmp_add_input_stream;

pub mod cmp_add_output_stream;

#[cfg(feature = "cmp_auth")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
pub mod cmp_auth;

pub mod cmp_derive;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc__esp__espidf)]
pub mod cmp_esp_adc;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc__esp__espidf)]
pub mod cmp_esp_gpio;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc__esp__espidf)]
pub mod cmp_esp_wifi;

pub mod cmp_external_fn_process;

#[cfg(feature = "cmp_http_client")]
#[cfg(any(
    aarch64_linux_android,
    aarch64__unknown__linux__gnu,
    x86_64__unknown__linux__gnu
))]
pub mod cmp_http_client;

#[cfg(feature = "cmp_http_client_wasm")]
#[cfg(wasm32__unknown__unknown)]
pub mod cmp_http_client_wasm;

#[cfg(feature = "cmp_http_server")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
pub mod cmp_http_server;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc__esp__espidf)]
pub mod cmp_http_server_esp;

#[cfg(feature = "cmp_influxdb")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
pub mod cmp_influxdb;

pub mod cmp_inject_periodic;

#[cfg(feature = "cmp_leptos")]
#[cfg(wasm32__unknown__unknown)]
pub mod cmp_leptos;

pub mod cmp_logger;

#[cfg(feature = "cmp_modbus_client")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
pub mod cmp_modbus_client;

#[cfg(feature = "cmp_plc")]
pub mod cmp_plc;

#[cfg(feature = "cmp_raspberrypi")]
#[cfg(aarch64__unknown__linux__gnu)]
pub mod cmp_raspberrypi_gpio;

#[cfg(feature = "cmp_redis_client")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
pub mod cmp_redis_client;

#[cfg(feature = "cmp_slint")]
#[cfg(any(
    aarch64_linux_android,
    aarch64__unknown__linux__gnu,
    x86_64__unknown__linux__gnu
))]
pub mod cmp_slint;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc__esp__espidf)]
pub mod cmp_storage_esp;

#[cfg(feature = "cmp_surrealdb")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
pub mod cmp_surrealdb;

#[cfg(feature = "cmp_system_info")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
pub mod cmp_system_info;

#[cfg(feature = "cmp_timescaledb")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
#[allow(unused_variables)]
#[allow(dead_code)]
pub mod cmp_timescaledb;

#[cfg(feature = "cmp_websocket_client")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
pub mod cmp_websocket_client;

#[cfg(feature = "cmp_websocket_client_wasm")]
#[cfg(wasm32__unknown__unknown)]
pub mod cmp_websocket_client_wasm;

#[cfg(feature = "cmp_websocket_server")]
#[cfg(any(aarch64__unknown__linux__gnu, x86_64__unknown__linux__gnu))]
pub mod cmp_websocket_server;

#[cfg(feature = "cmp_webstorage")]
#[cfg(wasm32__unknown__unknown)]
pub mod cmp_webstorage;
