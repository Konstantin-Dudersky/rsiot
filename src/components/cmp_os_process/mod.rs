//! Компонент cmp_os_process

mod component;
mod config;
mod error;
mod fn_process;
mod task_command;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::{Command, Config, ExecResult},
    error::Error,
};

type Result<T> = std::result::Result<T, Error>;
