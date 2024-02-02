use rsiot_messages_core::{msg_meta, IMessage, IMsgContentValue, MsgContent, MsgMeta};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgMeta)]
pub enum Message {
    TestMessage(MsgContent<i32>),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}
