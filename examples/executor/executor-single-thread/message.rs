use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, MsgKey, TimeToLiveValue};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Message {
    TestMessage(i32),
}

impl MsgDataBound for Message {
    fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
        TimeToLiveValue::Infinite
    }
}
