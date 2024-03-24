//! Компонент для подключения к Redis.
//!
//! Можно публиковать сообщения, подписываться на них, а также читать сообщения из кеша.

mod component;
mod config;
mod error;
mod fn_process;

pub use component::Cmp;
pub use config::{Config, ConfigFnInputItem};
