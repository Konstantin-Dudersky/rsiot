mod config;
mod error;
mod new;
mod process;
mod row;

/// Компонент сохранения сообщений в TimescaleDB
pub mod cmp_timescaledb_storing {
    pub use crate::{
        config::Config,
        new::new,
        row::{AggType, Row},
    };
}
pub use rsiot_component_core::ComponentChain;
pub use rsiot_messages_core::IMessage;
