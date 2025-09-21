//! Компонент MQTT-клиента для публикации и получения сообщений с брокера.

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::{COMPONENT_NAME, Cmp};
pub use config::{Config, ConfigPublish, ConfigSubscribe};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
