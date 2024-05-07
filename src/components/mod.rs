//! Компоненты

#[allow(dead_code)]
#[allow(unused_imports)]
mod _cmp_template;

pub mod cmp_add_input_stream;

pub mod cmp_add_output_stream;

#[cfg(feature = "cmp_auth")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_auth;

pub mod cmp_derive;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc_esp_espidf)]
pub mod cmp_esp_adc;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc_esp_espidf)]
pub mod cmp_esp_gpio;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc_esp_espidf)]
pub mod cmp_esp_i2c_master;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc_esp_espidf)]
pub mod cmp_esp_mqtt_client;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc_esp_espidf)]
pub mod cmp_esp_nvs;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc_esp_espidf)]
pub mod cmp_esp_wifi;

pub mod cmp_external_fn_process;

#[cfg(feature = "cmp_http_client")]
#[cfg(any(
    aarch64_linux_android,
    aarch64_unknown_linux_gnu,
    x8664_unknown_linux_gnu
))]
pub mod cmp_http_client;

#[cfg(feature = "cmp_http_client_wasm")]
#[cfg(wasm32_unknown_unknown)]
pub mod cmp_http_client_wasm;

#[cfg(feature = "cmp_http_server")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_http_server;

#[cfg(feature = "cmp_esp")]
#[cfg(riscv32imc_esp_espidf)]
pub mod cmp_http_server_esp;

#[cfg(feature = "cmp_influxdb")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_influxdb;

pub mod cmp_inject_periodic;

#[cfg(feature = "cmp_leptos")]
#[cfg(wasm32_unknown_unknown)]
pub mod cmp_leptos;

pub mod cmp_logger;

#[cfg(feature = "cmp_modbus_client")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_modbus_client;

#[cfg(feature = "cmp_mqtt_client")]
#[cfg(any(
    aarch64_linux_android,
    aarch64_unknown_linux_gnu,
    x8664_unknown_linux_gnu
))]
pub mod cmp_mqtt_client;

#[cfg(feature = "cmp_telegram")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_telegram;

#[cfg(feature = "cmp_plc")]
pub mod cmp_plc;

#[cfg(feature = "cmp_raspberrypi")]
#[cfg(aarch64_unknown_linux_gnu)]
pub mod cmp_raspberrypi_gpio;

#[cfg(feature = "cmp_raspberrypi")]
#[cfg(aarch64_unknown_linux_gnu)]
pub mod cmp_raspberrypi_i2c_master;

#[cfg(feature = "cmp_redis_client")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_redis_client;

#[cfg(feature = "cmp_slint")]
#[cfg(any(
    aarch64_linux_android,
    aarch64_unknown_linux_gnu,
    x8664_unknown_linux_gnu
))]
pub mod cmp_slint;

#[cfg(feature = "cmp_surrealdb")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_surrealdb;

#[cfg(feature = "cmp_system_info")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_system_info;

#[cfg(feature = "cmp_timescaledb")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
#[allow(unused_variables)]
#[allow(dead_code)]
pub mod cmp_timescaledb;

#[cfg(feature = "cmp_websocket_client")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_websocket_client;

#[cfg(feature = "cmp_websocket_client_wasm")]
#[cfg(wasm32_unknown_unknown)]
pub mod cmp_websocket_client_wasm;

#[cfg(feature = "cmp_websocket_server")]
#[cfg(any(aarch64_unknown_linux_gnu, x8664_unknown_linux_gnu))]
pub mod cmp_websocket_server;

#[cfg(feature = "cmp_webstorage")]
#[cfg(wasm32_unknown_unknown)]
pub mod cmp_webstorage;
