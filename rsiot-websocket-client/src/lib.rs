mod config;
mod error;
mod fn_process;
mod new;

pub use error::Error;
pub use rsiot_component_core::ComponentChain;
pub use rsiot_messages_core::IMessage;

/// Компонент для подключения через websocket server.
pub mod cmp_websocket_client {
    pub use crate::{config::Config, new::new};
}
