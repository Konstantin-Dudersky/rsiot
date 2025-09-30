//! Компонент для обмена данными по шине UART в режиме slave

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use crate::components_config::uart_general::{Baudrate, DataBits, Parity, StopBits};
pub use component::{COMPONENT_NAME, Cmp};
pub use config::*;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
type Buffer<T> = std::sync::Arc<tokio::sync::Mutex<T>>;
