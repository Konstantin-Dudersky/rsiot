mod async_task_utils;
mod config;
mod errors;
mod handle_ws_connection;
mod new;
mod process;

/// Компонент для подключения через websocket server.
///
/// Перенаправляет поток входящих сообщений подключенным вебсокет-клиентам.
/// Получает от клиентов новые сообщенияs
///
pub mod cmp_websocket_server {
    pub use crate::{config::Config, new::new};
}
