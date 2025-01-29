mod config;
mod device;
mod tasks;

use super::{BufferBound, Error, RequestResponseBound, Result};

pub use config::ConfigPeriodicRequest;
pub use device::DeviceBase;

type Buffer<T> = std::sync::Arc<tokio::sync::Mutex<T>>;
