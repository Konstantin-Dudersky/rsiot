use rsiot::message::MsgDataBound;
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
}

impl MsgDataBound for Custom {}
