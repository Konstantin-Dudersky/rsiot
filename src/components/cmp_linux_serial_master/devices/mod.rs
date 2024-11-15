//! Устройства для опроса через Uart

mod device_base;
mod test_device;

pub use test_device::TestDevice;

use super::Result;
