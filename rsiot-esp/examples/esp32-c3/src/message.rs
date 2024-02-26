use rsiot::message::{Deserialize, MsgDataBound, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    Button(bool),
    // SetLedColor(MsgContent<RGB8>), TODO - update rsiot
    TestFromHttpServer(u16),
    Relay2(bool),

    StorageI32(i32),
}

impl MsgDataBound for Custom {}
