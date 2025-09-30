use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, MsgKey};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    WifiConnected(bool),
    Counter(i32),
    Subscribe(i32),
}

impl MsgDataBound for Msg {}
