use rsiot_messages_core::{msg_meta, IMsgContentValue, MsgContent, MsgMeta};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgMeta)]
pub enum Message {
    TestMessage(MsgContent<i32>),
}
