mod config;
mod error;
mod new;
mod process;
mod row;

/// Компонент сохраниения сообщений в TimescaleDB
pub mod cmp_timescaledb_storing {
    pub use crate::{
        config::Config,
        new::new,
        row::{AggType, Row},
    };
}
