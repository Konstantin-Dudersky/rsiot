use rsiot::message::{msg_types, Deserialize, IMessage, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Message {
    InjectU16(msg_types::Value<u16>),
    InjectU16Output(msg_types::Value<u16>),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot::message::eav::EavModel> {
        vec![]
    }
}
