//! Компонент для периодического генерирования сообщений

mod component;
mod config;
mod error;
mod fn_process;

pub use {component::Cmp, config::Config, error::Error};
