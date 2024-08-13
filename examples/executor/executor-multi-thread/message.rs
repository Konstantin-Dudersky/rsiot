use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, TimeToLive};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Data {
    TestMessage(i32),
}

impl TimeToLive for Data {}

impl MsgDataBound for Data {}
