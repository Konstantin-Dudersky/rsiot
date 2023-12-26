//! Чтение входов и запись выходов GPIO микроконтроллера ESP.
//!
//! Тестируется с ESP32-C3 и ESP32-S3.
//!
//! TODO - В данный момент значение с пинов считывается циклически. Возможно, стоит переделать на
//! считывание по подписке.

pub mod hardware_tasks;
mod http_server;

pub use http_server::cmp_http_server_esp;
