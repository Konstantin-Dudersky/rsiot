//! Задачи, которые можно использовать в других компонентах

pub mod change_mpsc_msg;

#[cfg(any(feature = "cmp_linux_can", feature = "cmp_esp"))]
pub mod cmp_can_general;

pub(crate) mod cmp_http_client;
pub mod filter_identical_data;
pub mod filter_send_periodically;
pub mod fn_process_master;
pub mod mpsc_to_broadcast;
pub mod mpsc_to_msgbus;
pub mod msgbus_to_broadcast;
pub mod msgbus_to_mpsc;
pub mod msgbus_to_mpsc_unbounded;

#[cfg(any(
    feature = "cmp_websocket_client",
    feature = "cmp_websocket_client_wasm"
))]
pub mod cmp_websocket_client_general;
