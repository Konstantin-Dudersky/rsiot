mod config;
mod device;
mod response_result;
mod tasks;

#[cfg(test)]
mod tests;

use super::{BufferBound, Error, FieldbusDiagMsg, RequestResponseBound, Result};

pub use {config::ConfigPeriodicRequest, device::DeviceBase, response_result::ResponseResult};

type Buffer<T> = std::sync::Arc<tokio::sync::Mutex<T>>;
