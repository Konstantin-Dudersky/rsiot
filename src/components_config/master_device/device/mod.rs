mod config;
mod device;
mod device_state;
mod response_result;
mod tasks;

#[cfg(test)]
mod tests;

use super::{BufferBound, Error, RequestResponseBound, Result};

pub use {
    config::ConfigPeriodicRequest,
    device::DeviceBase,
    device_state::{ConfigDeviceStateOutput, DeviceState},
    response_result::ResponseResult,
};

type Buffer<T> = std::sync::Arc<tokio::sync::Mutex<T>>;
type DeviceStateType = std::sync::Arc<tokio::sync::Mutex<device_state::DeviceState>>;
