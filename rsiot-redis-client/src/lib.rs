#![cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]

mod component;
mod config;
mod error;
mod fn_process;

pub mod cmp_redis_client {
    pub use crate::component::Cmp;
    pub use crate::config::{Config, ConfigAlias};
}

// TODO - вынести сериализацию / десериализацию сообщений в функции fn_input | fn_output
