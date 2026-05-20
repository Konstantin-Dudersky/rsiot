//! Компонент для создания сообщений один раз.
//!
//! # Примеры
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_inject_single/config_inject_single.rs")]
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
