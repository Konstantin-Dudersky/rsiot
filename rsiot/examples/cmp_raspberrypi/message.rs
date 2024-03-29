use rsiot::message::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    Input4State(bool),
    SetOutput2(bool),
}

impl MsgDataBound for Custom {}
