use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum ServerMessages {
    ServerCounter(u32),
    CounterFromClient(u8),
}
impl MsgDataBound for ServerMessages {}
