use rsiot_messages_core::{msg_types, Deserialize, IMessage, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Message {
    OutputValue(msg_types::Value<u16>),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}
