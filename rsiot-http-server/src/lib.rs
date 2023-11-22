mod config;
mod error;
mod new;
mod process;
mod route_message_get;
mod route_message_put;
mod shared_state;

/// Компонент для получения данных через HTTP server.
///
pub mod cmp_http_server {
    pub use crate::{config::Config, new::new};
}
