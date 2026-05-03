//! Компонент измерения скорости

mod component;
mod config;
mod error;
mod fn_process;
mod int_msg;
mod task_edge_detect;
mod task_send;
mod task_tick;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
};

type Result<T> = std::result::Result<T, Error>;
