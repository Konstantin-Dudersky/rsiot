use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

use super::cfg_plc::fb_main;

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    Filesystem(Filesystem),
    Plc(Plc),
    MsgInjectPeriodic(MsgInjectPeriodic),
}
impl MsgDataBound for Msg {}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Filesystem {
    MemoryStatic(fb_main::S),
}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Plc {
    Input(fb_main::I),
    Output(fb_main::Q),
    Static(fb_main::S),
}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MsgInjectPeriodic {
    Counter(u32),
}
