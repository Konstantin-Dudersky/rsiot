use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, MsgKey};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Data {
    TestMessage(i32),
}

impl MsgDataBound for Data {}
