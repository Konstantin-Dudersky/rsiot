use rsiot_messages_core::{msg_meta, Deserialize, IMessage, MsgContent, MsgMeta, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgMeta)]
pub enum Message {
    OutputValue(MsgContent<u16>),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}
