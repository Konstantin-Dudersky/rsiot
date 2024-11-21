//! Библиотека устройств для опроса по UART

pub mod test_device;

use super::{
    device_base::{ConfigPeriodicRequest, DeviceBase},
    DeviceTrait, Result, UartMessageRaw,
};
