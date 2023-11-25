mod config;
mod error;
mod new;
mod process;
mod routes;
mod shared_state;

#[doc = include_str!("../README.md")]
///
/// # Диаграмма
///
#[doc = include_str!("../doc/component-http-server.svg")]
///
/// # Пример
///
/// ```rust
#[doc = include_str!("../examples/http-server-example.rs")]
/// ```
///
pub mod cmp_http_server {
    pub use crate::{config::Config, new::new};
}
