//! Компонент cmp_os_process
//!
//! # Примеры
//!
//! ## Синхронизация времени с сервером NTP
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_os_process/config_os_process/cmp_chronyc.rs")]
//! ```

mod component;
mod config;
mod error;
mod fn_process;
pub mod helpers;
mod task_command;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::{Config, ConfigCommand, ExecResult},
    error::Error,
};
