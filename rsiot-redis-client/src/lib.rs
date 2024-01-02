mod config;
mod error;
mod fn_process;
mod new;

pub use rsiot_component_core::ComponentCollection;
pub use rsiot_messages_core::IMessage;

pub mod cmp_redis_client {
    pub use crate::config::Config;
    pub use crate::new::new;
}
