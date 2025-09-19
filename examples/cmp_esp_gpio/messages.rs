use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    GpioInput(bool),
    GpioOutput(bool),
}

impl MsgDataBound for Msg {}
