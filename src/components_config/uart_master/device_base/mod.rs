mod config;
mod device_base;
mod tasks;

use super::{Error, Result, UartMessage, UartMessageRaw};

pub use config::ConfigPeriodicRequest;
pub use device_base::DeviceBase;

type Buffer<T> = std::sync::Arc<tokio::sync::Mutex<T>>;
