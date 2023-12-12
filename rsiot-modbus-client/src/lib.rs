mod config;
mod errors;
mod fn_process;
mod new;
mod types;

pub use rsiot_component_core::ComponentChain;
pub use rsiot_messages_core::IMessage;

#[doc = include_str!("../README.md")]
///
/// # Диаграмма
///
#[doc = include_str!("../doc/component-modbus-client.svg")]
///
/// # Пример
///
/// ```rust
#[doc = include_str!("../examples/modbus_tcp_client.rs")]
/// ```
pub mod cmp_modbus_client {
    pub use crate::new::new;
    pub use rsiot_components_config::modbus_client::*;
}
