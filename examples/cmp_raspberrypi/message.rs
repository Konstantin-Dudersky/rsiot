use serde::{Deserialize, Serialize};

use rsiot::message::*;

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Custom {
    Input4State(bool),
    SetOutput2(bool),
}

impl MsgDataBound for Custom {}
