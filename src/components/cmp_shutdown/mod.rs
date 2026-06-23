//! Компонент для выключения системы
//!
//! # Структура
//!
#![doc = include_str!("doc/diagram.svg")]
//!
//! Компонент состоит из одной задачи InjectPeriodic. С периодом [Config::period] на основе функции
//! [Config::fn_periodic] формируются исходящие сообщения и передаются в шину MsgBus.
//!
//! # Конфигурация
//!
//! Конфигурация задаётся структурой [Config].
//!
//! # Примеры
//!
//! ## Пример 1
//!
//! Счётчик увеличивается каждые 100 миллисекунд и отправляется в шину MsgBus.
//!
//! Содержимое файла `config_/mod.rs`:
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_inject_periodic/config_inject_periodic.rs")]
//! ```

mod component;
mod config;
mod error;
mod fn_process;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
};

type Result<T> = std::result::Result<T, Error>;
