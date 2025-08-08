use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

use super::config_plc::fb_main;

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    Filesystem(Filesystem),
    Plc(Plc),
}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Filesystem {
    MemoryStatic(fb_main::S),
}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Plc {
    MemoryInput(fb_main::I),
    MemoryOutput(fb_main::Q),
    MemoryStatic(fb_main::S),
}

impl MsgDataBound for Msg {}
