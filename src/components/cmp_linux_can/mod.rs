//! Компонент cmp_linux_can для работы с интерфейсом CAN

mod can_filter;
mod can_frame;
mod can_id;
mod can_settings;
mod can_socket;
mod component;
mod config;
mod error;
mod fn_process;
mod task_interface_info;
mod task_recv_from_can;
mod task_send_to_can;

pub use {
    crate::components_config::can_general::*,
    component::{COMPONENT_NAME, Cmp},
    config::Config,
    error::Error,
};

type Result<T> = std::result::Result<T, Error>;
