use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    Counter(u32),
    OutputSvg(Vec<u8>),
}

impl MsgDataBound for Msg {}
