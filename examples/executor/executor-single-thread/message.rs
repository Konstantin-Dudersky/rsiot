use rsiot::message::{MsgDataBound, TimeToLive};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Message {
    TestMessage(i32),
}

impl MsgDataBound for Message {}

impl TimeToLive for Message {}
