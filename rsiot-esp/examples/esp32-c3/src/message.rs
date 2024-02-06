use rgb::RGB8;
use rsiot::message::{
    msg_meta, Deserialize, IMessage, IMsgContentValue, MsgContent, MsgMeta, Serialize,
};

#[derive(Clone, Debug, Deserialize, MsgMeta, PartialEq, Serialize)]
pub enum Message {
    Button(MsgContent<bool>),
    // SetLedColor(MsgContent<RGB8>), TODO - update rsiot
    TestFromHttpServer(MsgContent<u16>),
    Relay2(MsgContent<bool>),

    StorageI32(MsgContent<i32>),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot::message::eav::EavModel> {
        vec![]
    }
}
