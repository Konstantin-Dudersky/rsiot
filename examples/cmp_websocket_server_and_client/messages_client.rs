use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum ClientMessages {
    ServerCounter(u32),
    CounterFromClient(u8),
    ConnectionState(bool),
}
impl MsgDataBound for ClientMessages {}
