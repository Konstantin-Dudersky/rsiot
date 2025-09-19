//! Компонент для подключения к Redis.
//!
//! НЕ ПОДДЕРЖИВАЕТСЯ И НЕ КОМПИЛИРУЕТСЯ
//!
//! Код оставлен на будущее, для адаптации к замене Redis на другую БД.

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigFnInputItem};
