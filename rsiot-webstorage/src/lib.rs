//! Хранение и загрузка сообщений используя LocalStorage браузера

#![cfg(target_arch = "wasm32")]

mod component;
mod config;
mod error;
mod fn_process;

pub use crate::component::Cmp;
pub use crate::config::{Config, ConfigKind};
pub use crate::error::Error;

type Result<T> = std::result::Result<T, error::Error>;
