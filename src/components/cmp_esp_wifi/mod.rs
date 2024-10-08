//! Шаблон компонента

mod component;
mod config;
mod error;
mod fn_process;
// mod wifi_manager;

pub use component::Cmp;
pub use config::{Config, ConfigAccessPoint, ConfigAuthMethod, ConfigClient};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
