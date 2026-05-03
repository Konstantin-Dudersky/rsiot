//! Компонент для вывода данных в консоль. Используется для отладки.
//!
//! # Структура
//!
#![doc = include_str!("doc/diagram.svg")]
//!
//! Компонент состоит из одной задачи Input. На основе входящих сообщений из шины MsgBus формируется
//! строка для вывода в консоль.
//!
//! # Конфигурация
//!
//! Конфигурация задаётся структурой [Config].
//!
//! # Примеры
//!
//! ## Пример 1
//!
//! Содержимое файла `config_logger/mod.rs`:
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_inject_periodic/config_logger.rs")]
//! ```

mod component;
mod config;
mod error;
mod fn_process;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
    tracing::Level,
};
