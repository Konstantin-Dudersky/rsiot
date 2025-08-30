//! Компонент для логгирования сообщений

mod component;
mod config;
mod error;
mod fn_process;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
    tracing::Level,
};
