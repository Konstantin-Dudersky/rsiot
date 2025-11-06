//! Компонент для создания сообщений на основе других сообщений

mod buffer_bound;
mod component;
mod config;
mod error;
mod fn_process;
mod internal_message;
mod task_input;
mod task_output;
mod task_period;

pub use {
    buffer_bound::BufferBound,
    component::{COMPONENT_NAME, Cmp},
    config::{Config, ConfigOutputSend},
    error::Error,
};
