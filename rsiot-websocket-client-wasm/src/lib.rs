#[cfg(feature = "single-thread")]
mod component;
#[cfg(feature = "single-thread")]
mod config;
#[cfg(feature = "single-thread")]
mod error;
#[cfg(feature = "single-thread")]
mod fn_process;

#[cfg(feature = "single-thread")]
pub mod cmp_websocket_client_wasm {
    pub use super::{component::Cmp, config::Config};
}
