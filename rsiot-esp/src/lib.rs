//! Компоненты для работы с микроконтроллерами ESP32.
//!
//! Тестируется с ESP32-C3 и ESP32-S3.
//!
//! TODO - В данный момент значение с пинов считывается циклически. Возможно, стоит переделать на
//! считывание по подписке.

pub mod hardware_tasks;
mod http_server;
mod storage;

// pub use http_server::cmp_http_server_esp;
pub use storage::cmp_storage_esp;
