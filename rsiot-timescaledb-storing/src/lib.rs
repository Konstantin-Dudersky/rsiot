mod config;
mod error;
mod fn_process;
mod model;
mod new;

/// Компонент сохранения сообщений в TimescaleDB
pub mod cmp_timescaledb_storing {
    pub use crate::{config::Config, new::new};
}
pub use rsiot_component_core::ComponentCollection;
pub use rsiot_messages_core::IMessage;
