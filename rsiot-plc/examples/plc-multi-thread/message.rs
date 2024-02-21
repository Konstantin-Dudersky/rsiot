use rsiot_messages_core::{message_v2::MsgDataBound, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Data {
    OutputValue(u16),
}

impl MsgDataBound for Data {}
