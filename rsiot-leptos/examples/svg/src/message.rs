use rsiot::message::*;

#[derive(Clone, Debug, Deserialize, MsgMeta, PartialEq, Serialize)]
pub enum Message {
    U16_0_100(MsgContent<u16>),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<eav::EavModel> {
        vec![]
    }
}
