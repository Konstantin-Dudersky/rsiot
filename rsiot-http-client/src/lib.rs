mod config;
mod error;
mod fn_process;
mod new;
mod types;

pub use rsiot_component_core::ComponentCollection;
pub use rsiot_messages_core::IMessage;

#[doc = include_str!("../README.md")]
///
/// # Диаграмма
///
// #[doc = include_str!("../doc/component-modbus-client.svg")]
///
/// # Пример
///
/// ```rust
#[doc = include_str!("../examples/http_client.rs")]
/// ```
pub mod cmp_http_client {
    pub use crate::config::config;
    pub use crate::new::new;
}
