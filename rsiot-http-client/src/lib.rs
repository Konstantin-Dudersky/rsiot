mod component;
mod config;
mod error;
mod fn_process;
mod types;

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
    pub use crate::component::Cmp;
    pub use crate::config::config;
}
