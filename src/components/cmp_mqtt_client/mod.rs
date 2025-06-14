//! Компонент MQTT-клиента для публикации и получения сообщений с брокера.
//!
//! # Примеры
//!
//! ## Только публикация данных
//! ```
#![doc = include_str!("../../../examples/cmp_mqtt_client/config_mqtt_server_publish.rs")]
//! ```
//!
//! ## Только получение данных
//! ```
#![doc = include_str!("../../../examples/cmp_mqtt_client/config_mqtt_server_subscribe.rs")]
//! ```

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::Cmp;
pub use config::{Config, ConfigPublish, ConfigSubscribe};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
