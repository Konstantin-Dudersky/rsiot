mod component;
mod config;
mod error;
mod fn_process;
mod model;

/// Компонент сохранения сообщений в TimescaleDB
pub mod cmp_timescaledb_storing {
    pub use crate::{component::Cmp, config::Config};
}
