use serde::{Deserialize, Serialize};

use crate::message::{MsgDataBound, MsgKey};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    ServerCounter(u32),
    CounterFromClient(u8),
}
impl MsgDataBound for Msg {}
