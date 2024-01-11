//! HTTP-клиент для платформы WASM

mod component;
mod config;
mod error;
mod fn_process;

pub mod cmp_http_client_wasm {
    pub use crate::component::Cmp;
    pub use crate::config::config;
}

type Result<T, TMessage> = std::result::Result<T, error::Error<TMessage>>;
