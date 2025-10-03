//! Компонент cmp_os_process

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
