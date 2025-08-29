//! Компонент для отправки сообщений в побочный поток

mod component;
mod config;
mod error;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
};
