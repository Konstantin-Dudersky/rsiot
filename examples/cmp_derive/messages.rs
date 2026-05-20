use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    Counter1(u32),
    Counter2(u32),
    DeriveMessage { counter1: u32, counter2: u32 },
}

impl MsgDataBound for Msg {}
