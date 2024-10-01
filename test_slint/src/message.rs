use rsiot::message::{example_service::Service, MsgDataBound};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    HostName(String),
    OsVesion(String),
    Eth0Mac(String),
    Wlan0Mac(String),
    CpuUsage(String),
    CpuTemp(String),

    Memory(String),
    Swap(String),

    DiskDevSda1(String),
    DiskDevSda2(String),

    GpioTab(bool),
    Gpio1(bool),
    Gpio2(bool),
    GpioBackspace(bool),
}

impl MsgDataBound for Custom {
    type TService = Service;
}
