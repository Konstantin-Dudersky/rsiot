#![cfg(any(
    target = "x86_64-unknown-linux-gnu",
    target = "aarch64-unknown-linux-gnu"
))]

mod component;
mod config;
mod error;
mod fn_process;
mod model;

/// Компонент сохранения сообщений в TimescaleDB
pub mod cmp_timescaledb_storing {
    pub use crate::{component::Cmp, config::Config};
}
