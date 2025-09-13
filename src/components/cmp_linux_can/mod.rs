//! Компонент cmp_linux_can для работы с интерфейсом CAN

mod component;
mod config;
mod error;
mod fn_process;
mod task_recv_from_can;
mod task_send_to_can;

pub use {
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
};

type Result<T> = std::result::Result<T, Error>;
