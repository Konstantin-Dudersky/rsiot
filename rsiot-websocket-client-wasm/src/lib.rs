#![cfg(all(target_arch = "wasm32", feature = "single-thread"))]

mod component;
mod config;
mod error;
mod fn_process;

pub mod cmp_websocket_client_wasm {
    pub use super::{component::Cmp, config::Config};
}

type Result = std::result::Result<(), error::Error>;
