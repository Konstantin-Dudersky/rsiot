//! Компонент измерения скорости

mod component;
mod config;
mod error;
mod fn_process;
mod task_calculate;
mod task_edge_detect;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
};

type Result<T> = std::result::Result<T, Error>;
