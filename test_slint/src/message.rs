use rsiot::message::MsgDataBound;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    HostName(String),
    OsVesion(String),
    Eth0Mac(String),
    Wlan0Mac(String),
}

impl MsgDataBound for Custom {}
