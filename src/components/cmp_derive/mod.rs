//! Компонент для создания сообщений на основе других сообщений
//!
//! # Структура
//!
#![doc = include_str!("doc/diagram.svg")]
//!
//! # Конфигурация
//!
//! Конфигурация задаётся структурой [Config].
//!
//! # Примеры
//!
//! ## Пример 1
//!
//! Объединить показания двух счётчиков
//!
//! Содержимое файла `config_derive/mod.rs`:
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_derive/config_derive.rs")]
//! ```

mod buffer_bound;
mod component;
mod config;
mod error;
mod fn_process;
mod internal_message;
mod task_input;
mod task_output;
mod task_period;

pub use {
    buffer_bound::BufferBound,
    component::{COMPONENT_NAME, Cmp},
    config::{Config, ConfigOutputSend},
    error::Error,
};
