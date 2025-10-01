//! Шаблон компонента

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
