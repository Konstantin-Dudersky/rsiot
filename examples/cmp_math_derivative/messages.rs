use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, MsgKey, ValueTime};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    InputValue(ValueTime),
    OutputValue(f64),
}

impl MsgDataBound for Msg {}
