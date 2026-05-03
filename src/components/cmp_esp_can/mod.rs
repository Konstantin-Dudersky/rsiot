//! Компонент cmp_esp_can

mod can_filter;
mod can_frame;
mod can_id;
mod can_timing;
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
