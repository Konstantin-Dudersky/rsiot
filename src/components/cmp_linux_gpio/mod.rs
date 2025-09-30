//! Компонент для работы с GPIO на Linux

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::{Config, ConfigGpioInput, ConfigGpioOutput},
    error::Error,
};

type Result<T> = std::result::Result<T, Error>;
