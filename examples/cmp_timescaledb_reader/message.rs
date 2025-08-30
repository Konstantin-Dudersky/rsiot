use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, MsgKey, ValueTime};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    MTsdbReader(MTsdbReader),
}

impl MsgDataBound for Msg {}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MTsdbReader {
    AccelX(ValueTime),
}
