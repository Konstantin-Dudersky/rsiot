//! Задачи, которые можно использовать в других компонентах

pub mod change_mpsc_msg;

#[cfg(any(feature = "cmp_linux_can", feature = "cmp_esp"))]
pub(crate) mod cmp_can_general;

#[cfg(any(
    feature = "cmp_websocket_client",
    feature = "cmp_websocket_client_wasm"
))]
pub(crate) mod cmp_websocket_client_general;

#[cfg(any(
    feature = "cmp_http_client",
    feature = "cmp_esp",
    feature = "cmp_http_client_wasm"
))]
pub(crate) mod cmp_http_client;

#[cfg(any(feature = "cmp_esp", feature = "cmp_mqtt_client"))]
pub(crate) mod cmp_mqtt_genral;

pub mod filter_identical_data;
pub mod filter_send_periodically;
pub mod fn_process_master;
pub mod mpsc_to_broadcast;
pub mod mpsc_to_msgbus;
pub mod mpsc_to_msgbus_new;
pub mod msgbus_to_broadcast;
pub mod msgbus_to_mpsc;
pub mod msgbus_to_mpsc_new;
pub mod msgbus_to_mpsc_unbounded;
