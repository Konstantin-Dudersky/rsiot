use rgb::RGB8;
use rsiot::message::{msg_types::Value, Deserialize, IMessage, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Message {
    Button(Value<bool>),
    SetLedColor(Value<RGB8>),
    TestFromHttpServer(u16),
    Relay2(Value<bool>),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot::message::eav::EavModel> {
        vec![]
    }
}
